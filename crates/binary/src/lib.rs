//! The HEE HOO binary interface: the VM's instruction set and the trait
//! every runnable program implements.
//!
//! This crate is the contract between program loaders (like
//! `stupid_binary`), assemblers (like `stupid_assembler`) and the VM
//! (`interpreter`). Neither side needs to know about the other, they just
//! both need to know about this.
//!
//! [`Instruction`] is an already-decoded instruction. Turning it into (or
//! out of) a 16-bit machine word is a file format's problem, not ours.
//! [`Binary`] is the trait any loaded program implements so the VM can run
//! it, no matter how stupidly it was loaded.

/// A single decoded VM instruction.
///
/// Only a subset of the full instruction set described in the project
/// README is currently implemented.
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /// Stop execution of the current [`Binary`].
    Halt,
    /// Initialize a register to a literal value.
    ///
    /// The first field is the destination register index (2 bits when
    /// decoded, so `0..=3`); the second field is the value to store in it
    /// (10 bits when decoded, so `0..=1023`).
    Init(u8, u16),
}

/// A program the HEE HOO VM can run.
///
/// Implementors hand over the instructions, where to start, and what data
/// memory looks like before the show begins. How they got that stuff is
/// their own business (see `stupid_binary::StupidBinary` for one
/// deeply unserious approach).
pub trait Binary {
    /// Returns the instruction pointer value execution should start at.
    ///
    /// # Returns
    /// A `u16` address into the instruction sequence.
    fn init_start_ptr(&self) -> u16;

    /// Returns the instruction stored at `address`.
    ///
    /// # Parameters
    /// - `address` (`u16`): index into the instruction sequence.
    ///
    /// # Returns
    /// A copy of the [`Instruction`] at `address`.
    ///
    /// # Panics
    /// Implementations may panic if `address` doesn't point at an
    /// instruction.
    fn fetch_instruction(&self, address: u16) -> Instruction;

    /// Returns the values to preload into data memory, starting at
    /// address `0`.
    ///
    /// # Returns
    /// A `&[u16]` slice of initial memory cell values.
    fn initial_data(&self) -> &[u16];
}
