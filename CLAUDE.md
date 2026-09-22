# CLAUDE.md

Guidance for Claude Code (or other agents) working in this repository.

## Project

HEE HOO VM — a minimal, from-scratch 16-bit register virtual machine, built as a learning/toy project. See [README.md](README.md) for the target architecture and full instruction set.

## Structure

Cargo workspace with a single member crate:

- [Cargo.toml](Cargo.toml) — workspace manifest (`members = ["crates/*"]`)
- [crates/heehoo/](crates/heehoo/) — the VM crate
  - [src/main.rs](crates/heehoo/src/main.rs) — everything currently lives here: the `Instruction` enum, `Binary` (loaded program) struct, and `Vm` interpreter, plus a `main()` that runs a hard-coded demo program

There is no `lib.rs` yet — the crate is binary-only.

## Current state vs. design

The README documents the full intended instruction set (16 opcodes: `HALT`, `INIT`, `LOAD`, `LDDY`, `SAVE`, `SVDY`, `MOVE`, `WRITE`, `READ`, `ADD`, `AND`, `OR`, `NOT`, `JUMP`, `JPZR`, `JPOV`). Only `HALT` and `INIT` are implemented in `Vm::execute` so far. When implementing new instructions, keep the README's instruction table and this file in sync with what `Vm::execute` actually handles.

## Conventions

- Doc comments (`///` / `//!`) follow standard rustdoc conventions: short description, `# Parameters`/params via natural prose, `# Panics` for panicking behavior, `# Side Effects` where mutation isn't obvious from the signature.
- Keep [README.md](README.md)'s "Usage" and instruction-set-implemented-status notes accurate as more opcodes are added.
