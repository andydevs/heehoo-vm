//! HEE HOO VM: a minimal 16-bit register machine.
//!
//! This crate defines the program trait and instruction set
//! ([`binary::Binary`], [`binary::Instruction`]) and the interpreter itself
//! ([`vm::Vm`]). [`vm::Vm::execute`] runs any [`binary::Binary`] to
//! completion by fetching and dispatching instructions in a loop until a
//! [`binary::Instruction::Halt`] is reached. [`stupid_bin`] provides
//! [`stupid_bin::StupidBinary`], a [`binary::Binary`] loaded from a file on
//! disk.

mod binary;
mod stupid_bin;
mod vm;

use crate::stupid_bin::StupidBinary;
use vm::Vm;

/// Entry point: loads the program in `heehoo.bin` (relative to the current
/// working directory) and runs it on a fresh [`Vm`].
///
/// # Panics
/// Panics if `heehoo.bin` is missing or malformed (see
/// [`StupidBinary::load`]) or if execution fails (see [`Vm::execute`]).
fn main() {
    let binary = StupidBinary::load("heehoo.bin");
    let mut vm = Vm::default();
    vm.execute(binary);
}
