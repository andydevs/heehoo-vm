# CLAUDE.md

Guidance for Claude Code (or other agents) working in this repository.

## Project

HEE HOO VM — a minimal, from-scratch 16-bit register virtual machine, built as a learning/toy project. See [README.md](README.md) for the target architecture and full instruction set.

## Structure

Cargo workspace with a single member crate:

- [Cargo.toml](Cargo.toml) — workspace manifest (`members = ["crates/*"]`)
- [crates/heehoo/](crates/heehoo/) — the VM crate
  - [src/main.rs](crates/heehoo/src/main.rs) — module declarations plus a `main()` that loads `heehoo.bin` from the working directory and runs it
  - [src/binary.rs](crates/heehoo/src/binary.rs) — the `Instruction` enum (with `Instruction::parse_inst` word decoding) and the `Binary` trait (anything the VM can run: start pointer, instruction fetch, initial data)
  - [src/stupid_bin.rs](crates/heehoo/src/stupid_bin.rs) — `StupidBinary` (implements `Binary`) and `StupidBinary::load`, which parses a binary file (magic `markiplier`, which is checked, then version, section sizes, start pointer, instruction and data sections, all little-endian). Prints debug output while loading when `DEBUG_STUPIDLY` is on.
  - [src/vm.rs](crates/heehoo/src/vm.rs) — the `Vm` interpreter

There is no `lib.rs` yet. The crate is binary-only, with `main.rs` declaring the `binary`, `stupid_bin` and `vm` modules. The binary file format is documented in README.md and in the `stupid_bin` module docs, so keep both in sync.

## Current state vs. design

The README documents the full intended instruction set (14 opcodes, `0000`–`1101`: `HALT`, `INIT`, `LOAD`, `SAVE`, `MOVE`, `WRITE`, `READ`, `ADD`, `AND`, `OR`, `NOT`, `JUMP`, `JPZR`, `JPOV`). Instructions are 16-bit words: a 4-bit opcode, a 2-bit register argument and a 10-bit value/address argument. Only `HALT` and `INIT` are implemented so far (in `Instruction::parse_inst` and `Vm::execute`). Decoding any other opcode panics.

The VM has 4 registers and 1024 memory cells (`REG_COUNT`/`MEMCELL_COUNT` in [vm.rs](crates/heehoo/src/vm.rs)), matching the README. When implementing new instructions, keep the README's instruction table and this file in sync with what `Instruction::parse_inst` and `Vm::execute` actually handle.

## Conventions

- Doc comments (`///` / `//!`) follow standard rustdoc conventions: short description, `# Parameters`/params via natural prose, `# Panics` for panicking behavior, `# Side Effects` where mutation isn't obvious from the signature.
  - Provide examples for public facing functions
- Leave [README.md](README.md) to the user to update. Don't work on it. Just update code docs
- Don't touch the `.bin` or `.notbin.txt` files (e.g. `heehoo.bin`, `heehoo.notbin.txt`). Don't edit, regenerate or document them.
- Have fun with this. It's light hearted and meant to be humorous