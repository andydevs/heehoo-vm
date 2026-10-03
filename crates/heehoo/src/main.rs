//! HEE HOO VM: a minimal 16-bit register machine.
//!
//! This crate defines the loaded program representation and instruction set
//! ([`binary::Binary`], [`binary::Instruction`]) and the interpreter itself
//! ([`vm::Vm`]). [`vm::Vm::execute`] runs a [`binary::Binary`] to completion
//! by fetching and dispatching instructions in a loop until a
//! [`binary::Instruction::Halt`] is reached. [`stupid_bin`] loads a
//! [`binary::Binary`] from a binary file on disk.

mod binary;
mod stupid_bin;
mod vm;

use stupid_bin::load_binary_stupidly;
use vm::Vm;

/// Entry point: loads the program in `heehoo.bin` (relative to the current
/// working directory) and runs it on a fresh [`Vm`].
///
/// # Panics
/// Panics if `heehoo.bin` is missing or malformed (see
/// [`load_binary_stupidly`]) or if execution fails (see [`Vm::execute`]).
fn main() {
    let binary = load_binary_stupidly("heehoo.bin");

    let mut vm = Vm::default();
    vm.execute(binary);
}
