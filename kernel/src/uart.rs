//! UART driver for QEMU virt machine (NS16550A compatible)

use core::fmt::{self, Write};
use core::ptr::{addr_of, addr_of_mut};

/// UART base address for QEMU virt machine
const UART_BASE: usize = 0x1000_0000;

/// UART register offsets
mod regs {
    pub const THR: usize = 0; // Transmit Holding Register (write)
    pub const IER: usize = 1; // Interrupt Enable Register
    pub const FCR: usize = 2; // FIFO Control Register (write)
    pub const LCR: usize = 3; // Line Control Register
    pub const LSR: usize = 5; // Line Status Register
}

/// Line Status Register bits
mod lsr {
    pub const TX_EMPTY: u8 = 1 << 5; // Transmit holding register empty
}

/// UART driver
pub struct Uart {
    base: usize,
}

impl Uart {
    /// Create a new UART instance
    const fn new(base: usize) -> Self {
        Self { base }
    }

    /// Read a register
    fn read_reg(&self, offset: usize) -> u8 {
        unsafe { ((self.base + offset) as *const u8).read_volatile() }
    }

    /// Write a register
    fn write_reg(&self, offset: usize, value: u8) {
        unsafe { ((self.base + offset) as *mut u8).write_volatile(value) }
    }

    /// Initialize the UART
    pub fn init(&self) {
        // Disable interrupts
        self.write_reg(regs::IER, 0x00);

        // Enable FIFO, clear TX/RX queues
        self.write_reg(regs::FCR, 0x07);

        // Set 8 bits, no parity, 1 stop bit (8N1)
        self.write_reg(regs::LCR, 0x03);
    }

    /// Transmit a byte
    pub fn put_char(&self, c: u8) {
        // Wait for transmit holding register to be empty
        while self.read_reg(regs::LSR) & lsr::TX_EMPTY == 0 {}
        self.write_reg(regs::THR, c);
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.put_char(b'\r');
            }
            self.put_char(byte);
        }
        Ok(())
    }
}

/// Global UART instance
static mut UART: Uart = Uart::new(UART_BASE);

/// Initialize the UART
pub fn init() {
    // SAFETY: Single-threaded kernel initialization
    unsafe { (*addr_of!(UART)).init() };
}

/// Print macro implementation
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    // SAFETY: Single-threaded kernel, no concurrent access
    unsafe { (*addr_of_mut!(UART)).write_fmt(args).unwrap() };
}

/// Print without newline
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

/// Print with newline
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
