//! The loaded program representation and the VM's instruction set.
//!
//! [`Instruction`] is a decoded instruction; [`Instruction::from_u16`] decodes
//! one from its 16-bit machine-word encoding (4-bit opcode, 2-bit first
//! argument, 10-bit second argument). [`Binary`] is a whole program as loaded
//! into the VM.

/// Opcode for [`Instruction::Halt`].
const CODE_HALT: u8 = 0b0000;
/// Opcode for [`Instruction::Init`].
const CODE_INIT: u8 = 0b0001;

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
    /// - `value` (`u16`): the encoded instruction word.
    ///
    /// # Returns
    /// The decoded [`Instruction`].
    ///
    /// # Panics
    /// Panics if the opcode doesn't belong to an implemented instruction
    /// (currently anything other than `HALT` or `INIT`).
    pub fn from_u16(value: u16) -> Self {
        let opcode: u8 = ((value & 0b1111_0000_0000_0000) >> 12)
            .try_into()
            .expect("What happened here???");
        let arg1: u8 = ((value & 0b0000_1100_0000_0000) >> 10)
            .try_into()
            .expect("Why dis no work?");
        let arg2: u16 = value & 0b0000_0011_1111_1111;
        match opcode {
            CODE_HALT => Self::Halt,
            CODE_INIT => Self::Init(arg1, arg2),
            _ => panic!("Opcode is an oopsie poopsie"),
        }
    }
}

/// A loaded program ready to be run by a [`crate::vm::Vm`].
pub struct Binary {
    /// The instruction sequence, indexed by instruction pointer.
    pub text: Vec<Instruction>,
    /// The instruction pointer value to start execution at.
    pub start_ptr: u16,
    /// Values to preload into data memory (starting at address `0`) before
    /// execution begins.
    pub initial_data: Vec<u16>,
}
