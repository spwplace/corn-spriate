//! Sprite - Bytecode VM and Verifier
//!
//! This crate provides:
//! - Bytecode instruction format
//! - Static verifier for bytecode safety
//! - Virtual machine for execution

#![cfg_attr(not(feature = "std"), no_std)]

pub mod bytecode;
pub mod module;

// Verifier and VM modules will be added in later phases
