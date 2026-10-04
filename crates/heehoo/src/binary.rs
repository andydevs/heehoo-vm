//! The loaded program representation and the VM's instruction set.
//!
//! [`Instruction`] is a decoded instruction; [`Instruction::parse_inst`]
//! decodes one from its 16-bit machine-word encoding (4-bit opcode, 2-bit
//! first argument, 10-bit second argument). [`Binary`] is the trait any
//! loaded program implements so the VM can run it, no matter how stupidly it
//! was loaded.

/// Bits of an instruction word holding the opcode (the top 4).
const OPCODE_MASK: u16 = 0b1111_0000_0000_0000;
/// How far to shift a masked opcode right to get its value.
const OPCODE_OFFSET: u8 = 12;
/// Bits of an instruction word holding the first (register) argument.
const ARG1_MASK: u16 = 0b0000_1100_0000_0000;
/// How far to shift a masked first argument right to get its value.
const ARG1_OFFSET: u8 = 10;
/// Bits of an instruction word holding the second (value/address) argument.
/// Already sitting at the bottom, so no shifting required. Lazy bits.
const ARG2_MASK: u16 = 0b0000_0011_1111_1111;

/// Opcode for [`Instruction::Halt`].
const OPCODE_HALT: u8 = 0b0000;
/// Opcode for [`Instruction::Init`].
const OPCODE_INIT: u8 = 0b0001;

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

impl Instruction {
    /// Decodes an instruction from its 16-bit machine-word encoding.
    ///
    /// The word is split as `oooo aa bbbbbbbbbb` (most significant bit
    /// first): a 4-bit opcode, a 2-bit first argument, and a 10-bit second
    /// argument. Arguments an instruction doesn't use are ignored.
    ///
    /// # Parameters
    /// - `address` (`u16`): where this word lives in the instruction section.
    ///   Only used to point fingers in the panic message.
    /// - `value` (`u16`): the encoded instruction word.
    ///
    /// # Returns
    /// The decoded [`Instruction`].
    ///
    /// # Panics
    /// Panics if the opcode doesn't belong to an implemented instruction
    /// (currently anything other than `HALT` or `INIT`).
    ///
    /// # Examples
    /// ```
    /// // INIT register 2 with 69: 0001 10 0001000101
    /// let inst = Instruction::parse_inst(0, 0b0001_1000_0100_0101);
    /// assert!(matches!(inst, Instruction::Init(2, 69)));
    ///
    /// // All zeroes is HALT. Nice.
    /// assert!(matches!(Instruction::parse_inst(1, 0), Instruction::Halt));
    /// ```
    pub fn parse_inst(address: u16, value: u16) -> Self {
        // Parse instruction opcode
        let opcode: u8 = ((value & OPCODE_MASK) >> OPCODE_OFFSET)
            .try_into()
            .expect("Could not convert opcode u16 to u8");

        // Parse instruction argument 1
        let arg1: u8 = ((value & ARG1_MASK) >> ARG1_OFFSET)
            .try_into()
            .expect("Could not convert instruction arg 1 u16 to u8");

        // Parse instruction argument 2
        let arg2: u16 = value & ARG2_MASK;

        // Match opcode
        match opcode {
            OPCODE_HALT => Self::Halt,
            OPCODE_INIT => Self::Init(arg1, arg2),
            _ => panic!("Invalid opcode: {address}: {opcode}"),
        }
    }
}

/// A program the [`crate::vm::Vm`] can run.
///
/// Implementors hand over the instructions, where to start, and what data
/// memory looks like before the show begins. How they got that stuff is
/// their own business (see [`crate::stupid_bin::StupidBinary`] for one
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
