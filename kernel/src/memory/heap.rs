//! Kernel heap allocator
//!
//! Simple bump allocator for kernel heap. Can be upgraded to a more
//! sophisticated allocator later.

use super::{PAGE_SIZE, frame};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::{self, addr_of_mut, null_mut};

/// Number of pages for initial heap
const HEAP_PAGES: usize = 64; // 256 KiB initial heap

/// Heap allocator state
struct BumpAllocator {
    start: usize,
    end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            next: 0,
            allocations: 0,
        }
    }

    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.next = start;
        self.allocations = 0;
    }

    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        // Align up
        let align = layout.align();
        let aligned = (self.next + align - 1) & !(align - 1);
        let end = aligned + layout.size();

        if end > self.end {
            return null_mut();
        }

        self.next = end;
        self.allocations += 1;
        aligned as *mut u8
    }

    fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        self.allocations -= 1;
        // Bump allocator doesn't actually free, but track count
        // In a real implementation, we'd use a free list
    }
}

/// Global kernel heap allocator
static mut HEAP: BumpAllocator = BumpAllocator::new();

/// Wrapper for GlobalAlloc trait
struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { (*addr_of_mut!(HEAP)).alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { (*addr_of_mut!(HEAP)).dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

/// Initialize the kernel heap
pub fn init() {
    // Allocate frames for heap
    let heap_frames = frame::alloc_contiguous(HEAP_PAGES)
        .expect("failed to allocate heap frames");

    let heap_start = heap_frames.as_usize();
    let heap_size = HEAP_PAGES * PAGE_SIZE;

    // Zero the heap memory
    unsafe {
        ptr::write_bytes(heap_start as *mut u8, 0, heap_size);
    }

    // Initialize the allocator
    unsafe {
        (*addr_of_mut!(HEAP)).init(heap_start, heap_size);
    }

    crate::println!("[heap] Initialized: {} KiB at {:#x}", heap_size / 1024, heap_start);
}

/// Get heap statistics
pub fn stats() -> (usize, usize, usize) {
    unsafe {
        let heap = &*addr_of_mut!(HEAP);
        let used = heap.next - heap.start;
        let total = heap.end - heap.start;
        (used, total, heap.allocations)
    }
}
