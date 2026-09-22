//! HEE HOO VM: a minimal 16-bit register machine.
//!
//! This module defines the VM's instruction set ([`Instruction`]), the loaded
//! program representation ([`Binary`]), and the interpreter itself ([`Vm`]).
//! [`Vm::execute`] runs a [`Binary`] to completion by fetching and dispatching
//! instructions in a loop until a [`Instruction::Halt`] is reached.

use crate::Instruction::Halt;

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
    /// The first field is the destination register index (`0..REG_COUNT`);
    /// the second field is the value to store in it.
    Init(u8, u16),
}

/// A loaded program ready to be run by a [`Vm`].
pub struct Binary {
    /// The instruction sequence, indexed by instruction pointer.
    pub text: Vec<Instruction>,
    /// The instruction pointer value to start execution at.
    pub start_ptr: u16,
    /// Values to preload into data memory (starting at address `0`) before
    /// execution begins.
    pub initial_data: Vec<u16>,
}

/// Number of general-purpose registers available to the [`Vm`].
const REG_COUNT: usize = 8;

/// Number of 16-bit words in the [`Vm`]'s data memory.
const MEMCELL_COUNT: usize = 512;

/// The VM interpreter: holds all mutable machine state (instruction pointer,
/// registers, and data memory) and executes [`Binary`] programs against it.
pub struct Vm {
    /// Instruction pointer
    inst_ptr: u16,
    /// Register array
    reg: [u16; REG_COUNT],
    /// Data Memory
    memory: Box<[u16; MEMCELL_COUNT]>,
}

impl Default for Vm {
    /// Creates a fresh [`Vm`] with the instruction pointer and all registers
    /// and memory cells zeroed.
    fn default() -> Self {
        let memory = Box::new([0; MEMCELL_COUNT]);
        let reg = [0; REG_COUNT];
        Self {
            inst_ptr: 0,
            reg,
            memory,
        }
    }
}

impl Vm {
    /// Runs `binary` to completion.
    ///
    /// Loads `binary.initial_data` into data memory starting at address `0`,
    /// sets the instruction pointer to `binary.start_ptr`, then repeatedly
    /// fetches and executes instructions until an [`Instruction::Halt`] is
    /// reached.
    ///
    /// # Parameters
    /// - `binary`: the program to run.
    ///
    /// # Side Effects
    /// Mutates `self`'s instruction pointer, registers, and data memory.
    ///
    /// # Panics
    /// Panics if execution reaches an instruction pointer with no
    /// corresponding entry in `binary.text` (see [`Vm::fetch_instruction`]).
    pub fn execute(&mut self, binary: Binary) {
        // Initialize data
        for (index, cell) in binary.initial_data.iter().enumerate() {
            self.memory[index] = *cell;
        }

        // Initialize instruction pointer
        self.inst_ptr = binary.start_ptr;

        // Start loop
        'main_loop: loop {
            // Get current instruction
            let instruction = self.fetch_instruction(&binary);

            // Handle instruction
            match instruction {
                Instruction::Halt => break 'main_loop,
                Instruction::Init(reg, val) => {
                    let idx = usize::from(reg);
                    self.reg[idx] = val;
                }
            }
        }
    }

    /// Reads the instruction at the current instruction pointer and
    /// advances the pointer by one.
    ///
    /// # Parameters
    /// - `binary`: the program being executed, whose `text` is indexed by
    ///   the current instruction pointer.
    ///
    /// # Returns
    /// A copy of the fetched [`Instruction`].
    ///
    /// # Side Effects
    /// Increments `self`'s instruction pointer.
    ///
    /// # Panics
    /// Panics if the instruction pointer is out of bounds for
    /// `binary.text`.
    fn fetch_instruction(&mut self, binary: &Binary) -> Instruction {
        let Some(inst) = binary.text.get(usize::from(self.inst_ptr)) else {
            panic!("Invalid instruction address {}", self.inst_ptr);
        };
        self.inst_ptr += 1;
        *inst
    }
}

/// Entry point: builds a small demo [`Binary`] that initializes three
/// registers and halts, then runs it on a fresh [`Vm`].
fn main() {
    use Instruction::*;
    let binary = Binary {
        text: vec![Init(0, 10), Init(1, 20), Init(2, 21), Halt],
        start_ptr: 0,
        initial_data: vec![],
    };
    let mut vm = Vm::default();
    vm.execute(binary);
}
