//! Sprite module format - contains functions and metadata

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
use alloc::vec::Vec;

use crate::bytecode::Instruction;

/// A function in a Sprite module
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name (for debugging)
    pub name: &'static str,
    /// Number of parameters
    pub param_count: u32,
    /// Number of local variables (including parameters)
    pub local_count: u32,
    /// Maximum stack depth needed
    pub max_stack: u32,
    /// The bytecode instructions
    #[cfg(feature = "std")]
    pub code: Vec<Instruction>,
    #[cfg(not(feature = "std"))]
    pub code: &'static [Instruction],
}

/// A complete Sprite module
#[derive(Debug)]
pub struct Module {
    /// Module name
    pub name: &'static str,
    /// Functions in this module
    #[cfg(feature = "std")]
    pub functions: Vec<Function>,
    #[cfg(not(feature = "std"))]
    pub functions: &'static [Function],
    /// Entry point function index
    pub entry: u32,
}

/// Magic bytes for Sprite bytecode files
pub const MAGIC: &[u8; 4] = b"SPRT";

/// Current bytecode version
pub const VERSION: u32 = 1;
