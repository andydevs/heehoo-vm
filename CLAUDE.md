# CLAUDE.md

Guidance for Claude Code (or other agents) working in this repository.

## Project

HEE HOO VM — a minimal, from-scratch 16-bit register virtual machine, built as a learning/toy project. See [README.md](README.md) for the target architecture and full instruction set.

## Structure

Cargo workspace with a single member crate:

- [Cargo.toml](Cargo.toml) — workspace manifest (`members = ["crates/*"]`)
- [crates/heehoo/](crates/heehoo/) — the VM crate
  - [src/main.rs](crates/heehoo/src/main.rs) — module declarations plus a `main()` that loads `heehoo.bin` from the working directory and runs it
  - [src/binary.rs](crates/heehoo/src/binary.rs) — the `Instruction` enum (with `Instruction::from_u16` word decoding) and the `Binary` (loaded program) struct
  - [src/stupid_bin.rs](crates/heehoo/src/stupid_bin.rs) — `load_binary_stupidly`, which parses a binary file (magic `markiplier`, version, section sizes, start pointer, instruction and data sections, all little-endian) into a `Binary`. Prints debug output while loading.
  - [src/vm.rs](crates/heehoo/src/vm.rs) — the `Vm` interpreter

There is no `lib.rs` yet. The crate is binary-only, with `main.rs` declaring the `binary`, `stupid_bin` and `vm` modules. The binary file format is documented in README.md and in the `stupid_bin` module docs, so keep both in sync.

## Current state vs. design

The README documents the full intended instruction set (14 opcodes, `0000`–`1101`: `HALT`, `INIT`, `LOAD`, `SAVE`, `MOVE`, `WRITE`, `READ`, `ADD`, `AND`, `OR`, `NOT`, `JUMP`, `JPZR`, `JPOV`). Instructions are 16-bit words: a 4-bit opcode, a 2-bit register argument and a 10-bit value/address argument. Only `HALT` and `INIT` are implemented so far (in `Instruction::from_u16` and `Vm::execute`). Decoding any other opcode panics.

The VM has 4 registers and 1024 memory cells (`REG_COUNT`/`MEMCELL_COUNT` in [vm.rs](crates/heehoo/src/vm.rs)), matching the README. When implementing new instructions, keep the README's instruction table and this file in sync with what `Instruction::from_u16` and `Vm::execute` actually handle.

## Conventions

- Doc comments (`///` / `//!`) follow standard rustdoc conventions: short description, `# Parameters`/params via natural prose, `# Panics` for panicking behavior, `# Side Effects` where mutation isn't obvious from the signature.
- Keep [README.md](README.md)'s "Usage" and instruction-set-implemented-status notes accurate as more opcodes are added.
- Don't touch the `.bin` or `.notbin.txt` files (e.g. `heehoo.bin`, `heehoo.notbin.txt`). Don't edit, regenerate or document them.
