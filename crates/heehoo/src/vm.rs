//! The VM interpreter.

use crate::binary::{Binary, Instruction};

/// Number of general-purpose registers available to the [`Vm`].
const REG_COUNT: usize = 4;

/// Number of 16-bit words in the [`Vm`]'s data memory.
const MEMCELL_COUNT: usize = 1024;

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
    /// - `binary` (`Binary`): the program to run.
    ///
    /// # Side Effects
    /// Mutates `self`'s instruction pointer, registers, and data memory.
    ///
    /// # Panics
    /// - If execution reaches an instruction pointer with no corresponding
    ///   entry in `binary.text` (see [`Vm::fetch_instruction`]).
    /// - If `binary.initial_data` is longer than data memory
    ///   ([`MEMCELL_COUNT`] words).
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
    /// - `binary` (`&Binary`): the program being executed, whose `text` is indexed by
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
    /// `binary.text`, or (in debug builds) if incrementing it overflows.
    fn fetch_instruction(&mut self, binary: &Binary) -> Instruction {
        let Some(inst) = binary.text.get(usize::from(self.inst_ptr)) else {
            panic!("Invalid instruction address {}", self.inst_ptr);
        };
        self.inst_ptr += 1;
        *inst
    }
}
