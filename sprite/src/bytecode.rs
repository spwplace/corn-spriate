//! Sprite bytecode instruction definitions

/// Bytecode instructions for the Sprite VM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    /// No operation
    Nop = 0x00,

    // Stack operations
    /// Push immediate i64 value
    PushI = 0x10,
    /// Pop and discard top of stack
    Pop = 0x11,
    /// Duplicate top of stack
    Dup = 0x12,
    /// Swap top two stack values
    Swap = 0x13,

    // Arithmetic (i64)
    /// Add top two values
    Add = 0x20,
    /// Subtract (second - top)
    Sub = 0x21,
    /// Multiply
    Mul = 0x22,
    /// Divide (second / top)
    Div = 0x23,
    /// Modulo (second % top)
    Mod = 0x24,
    /// Negate top value
    Neg = 0x25,

    // Comparison -> pushes 0 (false) or 1 (true)
    /// Equal
    Eq = 0x30,
    /// Not equal
    Ne = 0x31,
    /// Less than
    Lt = 0x32,
    /// Less than or equal
    Le = 0x33,
    /// Greater than
    Gt = 0x34,
    /// Greater than or equal
    Ge = 0x35,

    // Boolean operations
    /// Logical AND
    And = 0x40,
    /// Logical OR
    Or = 0x41,
    /// Logical NOT
    Not = 0x42,

    // Local variable access
    /// Load from local slot
    Load = 0x50,
    /// Store to local slot
    Store = 0x51,

    // Control flow
    /// Unconditional jump
    Jmp = 0x60,
    /// Jump if zero (false)
    Jz = 0x61,
    /// Jump if not zero (true)
    Jnz = 0x62,

    // Functions
    /// Call function by index
    Call = 0x70,
    /// Return from function
    Ret = 0x71,

    // System
    /// System call
    Syscall = 0x80,
    /// Halt execution
    Halt = 0xFF,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Nop),
            0x10 => Ok(Self::PushI),
            0x11 => Ok(Self::Pop),
            0x12 => Ok(Self::Dup),
            0x13 => Ok(Self::Swap),
            0x20 => Ok(Self::Add),
            0x21 => Ok(Self::Sub),
            0x22 => Ok(Self::Mul),
            0x23 => Ok(Self::Div),
            0x24 => Ok(Self::Mod),
            0x25 => Ok(Self::Neg),
            0x30 => Ok(Self::Eq),
            0x31 => Ok(Self::Ne),
            0x32 => Ok(Self::Lt),
            0x33 => Ok(Self::Le),
            0x34 => Ok(Self::Gt),
            0x35 => Ok(Self::Ge),
            0x40 => Ok(Self::And),
            0x41 => Ok(Self::Or),
            0x42 => Ok(Self::Not),
            0x50 => Ok(Self::Load),
            0x51 => Ok(Self::Store),
            0x60 => Ok(Self::Jmp),
            0x61 => Ok(Self::Jz),
            0x62 => Ok(Self::Jnz),
            0x70 => Ok(Self::Call),
            0x71 => Ok(Self::Ret),
            0x80 => Ok(Self::Syscall),
            0xFF => Ok(Self::Halt),
            _ => Err(()),
        }
    }
}

/// A single instruction with its operand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: i64,
}

impl Instruction {
    pub const fn new(opcode: Opcode) -> Self {
        Self { opcode, operand: 0 }
    }

    pub const fn with_operand(opcode: Opcode, operand: i64) -> Self {
        Self { opcode, operand }
    }
}
