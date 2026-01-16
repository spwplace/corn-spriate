//! Sv39 page table management
//!
//! RISC-V Sv39 uses 3-level page tables with 512 entries per level.
//! Virtual addresses: 39 bits (512 GiB address space)
//! Physical addresses: 56 bits

use super::{PhysAddr, VirtAddr, PAGE_SIZE};
use super::frame::{self, zero_frame};

/// Page table entry flags
#[repr(u64)]
pub enum PteFlags {
    Valid = 1 << 0,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    User = 1 << 4,
    Global = 1 << 5,
    Accessed = 1 << 6,
    Dirty = 1 << 7,
}

/// Common flag combinations
pub const PTE_RW: u64 = PteFlags::Valid as u64 | PteFlags::Read as u64 | PteFlags::Write as u64;
pub const PTE_RX: u64 = PteFlags::Valid as u64 | PteFlags::Read as u64 | PteFlags::Execute as u64;
pub const PTE_RWX: u64 = PTE_RW | PteFlags::Execute as u64;
pub const PTE_USER_RW: u64 = PTE_RW | PteFlags::User as u64;
pub const PTE_USER_RX: u64 = PTE_RX | PteFlags::User as u64;
pub const PTE_USER_RWX: u64 = PTE_RWX | PteFlags::User as u64;

/// A page table entry
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn new(ppn: usize, flags: u64) -> Self {
        Self(((ppn as u64) << 10) | flags)
    }

    pub const fn is_valid(&self) -> bool {
        self.0 & PteFlags::Valid as u64 != 0
    }

    pub const fn is_leaf(&self) -> bool {
        // A leaf entry has R, W, or X set
        self.0 & (PteFlags::Read as u64 | PteFlags::Write as u64 | PteFlags::Execute as u64) != 0
    }

    pub const fn ppn(&self) -> usize {
        ((self.0 >> 10) & 0xFFF_FFFF_FFFF) as usize
    }

    pub const fn flags(&self) -> u64 {
        self.0 & 0x3FF
    }

    pub fn phys_addr(&self) -> PhysAddr {
        PhysAddr::new(self.ppn() << 12)
    }
}

/// A page table (512 entries, 4 KiB)
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    /// Create an empty page table
    pub const fn empty() -> Self {
        Self {
            entries: [PageTableEntry::empty(); 512],
        }
    }

    /// Get entry at index
    pub fn get(&self, index: usize) -> PageTableEntry {
        self.entries[index]
    }

    /// Set entry at index
    pub fn set(&mut self, index: usize, entry: PageTableEntry) {
        self.entries[index] = entry;
    }
}

/// Root page table for kernel address space
static mut KERNEL_PAGE_TABLE: PageTable = PageTable::empty();

/// Get the kernel page table
pub fn kernel_page_table() -> *mut PageTable {
    core::ptr::addr_of_mut!(KERNEL_PAGE_TABLE)
}

/// Map a virtual address to a physical address
///
/// Creates intermediate page tables as needed.
pub fn map(
    root: &mut PageTable,
    vaddr: VirtAddr,
    paddr: PhysAddr,
    flags: u64,
) -> Result<(), &'static str> {
    let vpn = [vaddr.vpn0(), vaddr.vpn1(), vaddr.vpn2()];

    // Walk/create page table levels
    let mut table = root;

    // Level 2 -> Level 1
    let l2_entry = table.get(vpn[2]);
    let l1_table = if l2_entry.is_valid() {
        l2_entry.phys_addr().as_usize() as *mut PageTable
    } else {
        let frame = frame::alloc().ok_or("out of memory")?;
        zero_frame(frame);
        let entry = PageTableEntry::new(frame.page_number(), PteFlags::Valid as u64);
        table.set(vpn[2], entry);
        frame.as_usize() as *mut PageTable
    };

    // Level 1 -> Level 0
    table = unsafe { &mut *l1_table };
    let l1_entry = table.get(vpn[1]);
    let l0_table = if l1_entry.is_valid() {
        l1_entry.phys_addr().as_usize() as *mut PageTable
    } else {
        let frame = frame::alloc().ok_or("out of memory")?;
        zero_frame(frame);
        let entry = PageTableEntry::new(frame.page_number(), PteFlags::Valid as u64);
        table.set(vpn[1], entry);
        frame.as_usize() as *mut PageTable
    };

    // Level 0 - leaf entry
    table = unsafe { &mut *l0_table };
    let entry = PageTableEntry::new(paddr.page_number(), flags);
    table.set(vpn[0], entry);

    Ok(())
}

/// Map a range of pages (identity mapping)
pub fn map_range(
    root: &mut PageTable,
    start: usize,
    end: usize,
    flags: u64,
) -> Result<(), &'static str> {
    let mut addr = start & !(PAGE_SIZE - 1);
    while addr < end {
        map(root, VirtAddr::new(addr), PhysAddr::new(addr), flags)?;
        addr += PAGE_SIZE;
    }
    Ok(())
}

/// Activate a page table by writing to satp
pub fn activate(root: &PageTable) {
    let ppn = (root as *const _ as usize) >> 12;
    // satp format: MODE[63:60] | ASID[59:44] | PPN[43:0]
    // MODE = 8 for Sv39
    let satp = (8 << 60) | ppn;

    unsafe {
        core::arch::asm!(
            "csrw satp, {0}",
            "sfence.vma",
            in(reg) satp
        );
    }
}

/// Set up identity mapping for kernel
pub fn init_kernel_mapping() -> Result<(), &'static str> {
    unsafe extern "C" {
        static _heap_end: u8;
    }

    let kernel_start = 0x8000_0000usize;
    let kernel_end = core::ptr::addr_of!(_heap_end) as usize;

    // Identity map kernel memory region
    let root = unsafe { &mut *kernel_page_table() };
    map_range(root, kernel_start, kernel_end, PTE_RWX)?;

    // Map UART for I/O
    map(root, VirtAddr::new(0x1000_0000), PhysAddr::new(0x1000_0000), PTE_RW)?;

    Ok(())
}
