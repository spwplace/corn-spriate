//! Physical frame allocator
//!
//! Simple bitmap-based allocator for physical memory frames.

use super::{PhysAddr, PAGE_SIZE, page_ceil};
use crate::println;
use core::ptr::addr_of_mut;

/// Maximum number of frames we can track
const MAX_FRAMES: usize = 32768; // 128 MiB with 4K pages

/// Bitmap for tracking frame allocation
static mut FRAME_BITMAP: [u64; MAX_FRAMES / 64] = [0; MAX_FRAMES / 64];

/// Start of allocatable physical memory
static mut FRAME_START: usize = 0;
/// End of allocatable physical memory
static mut FRAME_END: usize = 0;
/// Number of allocatable frames
static mut FRAME_COUNT: usize = 0;
/// Number of allocated frames
static mut FRAMES_USED: usize = 0;

/// Initialize the frame allocator
pub fn init(heap_start: usize, heap_end: usize) {
    let start = page_ceil(heap_start);
    let end = heap_end & !(PAGE_SIZE - 1);
    let count = (end - start) / PAGE_SIZE;

    // SAFETY: Single-threaded initialization
    unsafe {
        FRAME_START = start;
        FRAME_END = end;
        FRAME_COUNT = count.min(MAX_FRAMES);
        FRAMES_USED = 0;

        // Clear bitmap
        let bitmap = &mut *addr_of_mut!(FRAME_BITMAP);
        for slot in bitmap.iter_mut() {
            *slot = 0;
        }
    }

    println!(
        "[frame] Initialized: {} frames ({} KiB)",
        unsafe { FRAME_COUNT },
        unsafe { FRAME_COUNT } * 4
    );
}

/// Allocate a single physical frame
pub fn alloc() -> Option<PhysAddr> {
    // SAFETY: Single-threaded kernel
    unsafe {
        let bitmap = &mut *addr_of_mut!(FRAME_BITMAP);
        let count = FRAME_COUNT;

        for (i, slot) in bitmap.iter_mut().enumerate() {
            if *slot != u64::MAX {
                // Find first zero bit
                let bit = (!*slot).trailing_zeros() as usize;
                let frame_idx = i * 64 + bit;

                if frame_idx >= count {
                    return None;
                }

                // Mark as allocated
                *slot |= 1 << bit;
                FRAMES_USED += 1;

                let addr = FRAME_START + frame_idx * PAGE_SIZE;
                return Some(PhysAddr::new(addr));
            }
        }
    }
    None
}

/// Allocate multiple contiguous physical frames
pub fn alloc_contiguous(count: usize) -> Option<PhysAddr> {
    if count == 0 {
        return None;
    }
    if count == 1 {
        return alloc();
    }

    // SAFETY: Single-threaded kernel
    unsafe {
        let bitmap = &*addr_of_mut!(FRAME_BITMAP);
        let total = FRAME_COUNT;

        // Simple linear search for contiguous region
        let mut start = 0;
        while start + count <= total {
            let mut found = true;
            for i in 0..count {
                let idx = start + i;
                let slot = idx / 64;
                let bit = idx % 64;
                if bitmap[slot] & (1 << bit) != 0 {
                    start = idx + 1;
                    found = false;
                    break;
                }
            }

            if found {
                // Mark all frames as allocated
                let bitmap = &mut *addr_of_mut!(FRAME_BITMAP);
                for i in 0..count {
                    let idx = start + i;
                    let slot = idx / 64;
                    let bit = idx % 64;
                    bitmap[slot] |= 1 << bit;
                }
                FRAMES_USED += count;

                let addr = FRAME_START + start * PAGE_SIZE;
                return Some(PhysAddr::new(addr));
            }
        }
    }
    None
}

/// Free a physical frame
pub fn free(addr: PhysAddr) {
    // SAFETY: Single-threaded kernel
    unsafe {
        let frame_addr = addr.as_usize();
        if frame_addr < FRAME_START || frame_addr >= FRAME_END {
            return;
        }

        let frame_idx = (frame_addr - FRAME_START) / PAGE_SIZE;
        let slot = frame_idx / 64;
        let bit = frame_idx % 64;

        let bitmap = &mut *addr_of_mut!(FRAME_BITMAP);
        bitmap[slot] &= !(1 << bit);
        FRAMES_USED -= 1;
    }
}

/// Free multiple contiguous frames
pub fn free_contiguous(addr: PhysAddr, count: usize) {
    for i in 0..count {
        free(PhysAddr::new(addr.as_usize() + i * PAGE_SIZE));
    }
}

/// Get allocation statistics
pub fn stats() -> (usize, usize) {
    unsafe { (FRAMES_USED, FRAME_COUNT) }
}

/// Zero out a frame
pub fn zero_frame(addr: PhysAddr) {
    let ptr = addr.as_usize() as *mut u8;
    unsafe {
        core::ptr::write_bytes(ptr, 0, PAGE_SIZE);
    }
}
