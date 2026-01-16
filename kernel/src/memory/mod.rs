//! Memory management subsystem
//!
//! Provides:
//! - Physical frame allocation
//! - Sv39 page table management
//! - Kernel heap allocation

pub mod frame;
pub mod page;
pub mod heap;

use crate::println;

/// Page size (4 KiB)
pub const PAGE_SIZE: usize = 4096;
/// Page size shift
pub const PAGE_SHIFT: usize = 12;

/// Align address down to page boundary
pub const fn page_floor(addr: usize) -> usize {
    addr & !(PAGE_SIZE - 1)
}

/// Align address up to page boundary
pub const fn page_ceil(addr: usize) -> usize {
    (addr + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

/// Physical address type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(pub usize);

/// Virtual address type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(pub usize);

impl PhysAddr {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub const fn as_usize(self) -> usize {
        self.0
    }

    pub const fn page_offset(self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    pub const fn page_number(self) -> usize {
        self.0 >> PAGE_SHIFT
    }
}

impl VirtAddr {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub const fn as_usize(self) -> usize {
        self.0
    }

    pub const fn page_offset(self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    /// Get VPN[0] - page table index level 0
    pub const fn vpn0(self) -> usize {
        (self.0 >> 12) & 0x1FF
    }

    /// Get VPN[1] - page table index level 1
    pub const fn vpn1(self) -> usize {
        (self.0 >> 21) & 0x1FF
    }

    /// Get VPN[2] - page table index level 2
    pub const fn vpn2(self) -> usize {
        (self.0 >> 30) & 0x1FF
    }
}

/// Initialize memory subsystem
pub fn init() {
    // Get memory boundaries from linker
    unsafe extern "C" {
        static _heap_start: u8;
        static _heap_end: u8;
    }

    let heap_start = core::ptr::addr_of!(_heap_start) as usize;
    let heap_end = core::ptr::addr_of!(_heap_end) as usize;

    println!("[memory] Heap region: {:#x} - {:#x}", heap_start, heap_end);
    println!("[memory] Available: {} KiB", (heap_end - heap_start) / 1024);

    // Initialize frame allocator
    frame::init(heap_start, heap_end);

    // Initialize kernel heap
    heap::init();

    println!("[memory] Memory subsystem initialized");
}
