//! HEE HOO VM: a minimal 16-bit register machine.
//!
//! This crate holds the interpreter itself ([`vm::Vm`]).
//! [`vm::Vm::execute`] runs any [`binary::Binary`] to completion
//! by fetching and dispatching instructions in a loop until a
//! [`binary::Instruction::Halt`] is reached. The program comes
//! from [`stupid_binary::StupidBinary`], which loads `example.bin` from the
//! current working directory. Make one with `stupid_assembler` first.

mod vm;

use stupid_binary::StupidBinary;
use vm::Vm;

/// Entry point: loads the program in `example.bin` (relative to the current
/// working directory) and runs it on a fresh [`Vm`].
///
/// # Panics
/// Panics if `example.bin` is missing or malformed (see
/// [`StupidBinary::load`]) or if execution fails (see [`Vm::execute`]).
fn main() {
    let binary = StupidBinary::load("example.bin");
    let mut vm = Vm::default();
    vm.execute(binary);
}
