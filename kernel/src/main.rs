#![no_std]
#![no_main]

extern crate alloc;

mod memory;
mod uart;

use alloc::vec::Vec;
use alloc::boxed::Box;
use core::arch::{asm, naked_asm};
use core::panic::PanicInfo;

/// Entry point - sets up stack and jumps to kernel_main
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        // Set up stack pointer
        "la sp, _stack_start",
        // Clear BSS section
        "la t0, _bss_start",
        "la t1, _bss_end",
        "1:",
        "bgeu t0, t1, 2f",
        "sd zero, 0(t0)",
        "addi t0, t0, 8",
        "j 1b",
        "2:",
        // Jump to Rust code
        "call kernel_main",
        // Halt if kernel_main returns
        "3:",
        "wfi",
        "j 3b",
    )
}

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> ! {
    uart::init();

    println!();
    println!("  ██████╗ ██████╗ ██████╗ ███╗   ██╗");
    println!(" ██╔════╝██╔═══██╗██╔══██╗████╗  ██║");
    println!(" ██║     ██║   ██║██████╔╝██╔██╗ ██║");
    println!(" ██║     ██║   ██║██╔══██╗██║╚██╗██║");
    println!(" ╚██████╗╚██████╔╝██║  ██║██║ ╚████║");
    println!("  ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝");
    println!();
    println!("       ███████╗██████╗ ██████╗ ██╗████████╗███████╗");
    println!("       ██╔════╝██╔══██╗██╔══██╗██║╚══██╔══╝██╔════╝");
    println!("       ███████╗██████╔╝██████╔╝██║   ██║   █████╗  ");
    println!("       ╚════██║██╔═══╝ ██╔══██╗██║   ██║   ██╔══╝  ");
    println!("       ███████║██║     ██║  ██║██║   ██║   ███████╗");
    println!("       ╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝   ╚═╝   ╚══════╝");
    println!();
    println!("corn-sprite kernel v0.1.0");
    println!("RISC-V 64-bit | QEMU virt machine");
    println!();

    // Initialize memory subsystem
    memory::init();
    println!();

    // Test heap allocation
    test_heap();
    println!();

    println!("[kernel] Boot successful!");
    println!("[kernel] Entering idle loop...");

    loop {
        unsafe { asm!("wfi") };
    }
}

fn test_heap() {
    println!("[test] Testing heap allocation...");

    // Test Box
    let boxed = Box::new(42i64);
    println!("[test] Box<i64> = {}", *boxed);

    // Test Vec
    let mut vec: Vec<i64> = Vec::new();
    for i in 0..10 {
        vec.push(i * i);
    }
    println!("[test] Vec capacity: {}, len: {}", vec.capacity(), vec.len());
    println!("[test] Vec contents: {:?}", &vec[..]);

    // Check heap stats
    let (used, total, allocs) = memory::heap::stats();
    println!("[test] Heap: {} / {} bytes, {} allocations", used, total, allocs);

    // Check frame stats
    let (frames_used, frames_total) = memory::frame::stats();
    println!("[test] Frames: {} / {} used", frames_used, frames_total);

    println!("[test] Heap tests passed!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    if let Some(location) = info.location() {
        println!("at {}:{}:{}", location.file(), location.line(), location.column());
    }
    if let Some(msg) = info.message().as_str() {
        println!("message: {}", msg);
    }
    loop {
        unsafe { asm!("wfi") };
    }
}
