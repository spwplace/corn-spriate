/* Linker script for QEMU virt machine (RISC-V 64-bit) */

OUTPUT_ARCH(riscv)
ENTRY(_start)

/* QEMU virt machine RAM starts at 0x80000000 */
MEMORY {
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS {
    . = ORIGIN(RAM);

    /* Code section */
    .text : {
        *(.text.entry)    /* Entry point first */
        *(.text .text.*)
    } > RAM

    /* Read-only data */
    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } > RAM

    /* Initialized data */
    .data : ALIGN(4K) {
        _data_start = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        _data_end = .;
    } > RAM

    /* BSS (uninitialized data) */
    .bss : ALIGN(4K) {
        _bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        _bss_end = .;
    } > RAM

    /* Kernel stack */
    . = ALIGN(4K);
    _stack_end = .;
    . += 64K;
    _stack_start = .;

    /* Heap starts after stack */
    . = ALIGN(4K);
    _heap_start = .;
    _heap_end = ORIGIN(RAM) + LENGTH(RAM);

    /* Discard unwanted sections */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}
