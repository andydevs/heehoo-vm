# CLAUDE.md

Guidance for Claude Code (or other agents) working in this repository.

## Project

HEE HOO VM — a minimal, from-scratch 16-bit register virtual machine, built as a learning/toy project. See [README.md](README.md) for the target architecture and full instruction set.

## Structure

Cargo workspace with four member crates:

- [Cargo.toml](Cargo.toml) — workspace manifest (`members = ["crates/*"]`)
- [crates/binary/](crates/binary/) — library crate, the binary interface shared by loaders and the VM. No dependencies.
  - [src/lib.rs](crates/binary/src/lib.rs) — the `Instruction` enum (decoded instructions) and the `Binary` trait (anything the VM can run: start pointer, instruction fetch, initial data). Has no word encoding/decoding of its own.
- [crates/stupid_binary/](crates/stupid_binary/) — library crate, depends on `binary`
  - [src/lib.rs](crates/stupid_binary/src/lib.rs) — `StupidBinary` (implements `Binary` and `Default`, with public `text`, `start_ptr` and `initial_data` fields). Reads files with `StupidBinary::load` (magic `markiplier`, which is checked, then version, section sizes, start pointer, instruction and data sections, all little-endian) and writes them with `StupidBinary::to_bytes` (always writes version `0` and start pointer `0`). Also holds the private instruction word codec: `integer_to_inst` (decode) and `inst_to_integer` (encode). Prints debug output while loading and writing when `DEBUG_STUPIDLY` is on.
- [crates/interpreter/](crates/interpreter/) — binary crate, the VM itself; depends on `binary` and `stupid_binary`
  - [src/main.rs](crates/interpreter/src/main.rs) — declares the `vm` module, plus a `main()` that loads `example.bin` from the working directory with `StupidBinary::load` and runs it
  - [src/vm.rs](crates/interpreter/src/vm.rs) — the `Vm` interpreter
- [crates/stupid_assembler/](crates/stupid_assembler/) — binary crate, the assembler; depends on `binary`, `stupid_binary` and `regex`
  - [src/main.rs](crates/stupid_assembler/src/main.rs) — reads `example.hasm` from the working directory, parses it into a `StupidBinary` and writes `example.bin` with `StupidBinary::to_bytes`. Assembly syntax: a `[TEXT]` section header, then one instruction per line (`halt`, `init <reg> <val>`, case-insensitive). Prints every line it parses.

Run a specific binary with `cargo run -p interpreter` or `cargo run -p stupid_assembler`. A plain `cargo run` can't pick between them.

`binary` must not depend on the other crates, so its docs mention them in plain backticks rather than intra-doc links. The library crates' doc examples run as doctests (`cargo test`), so keep them compiling. Private functions can't be called from doctests, so give them unit tests instead of runnable examples. The examples in `interpreter` and `stupid_assembler` aren't compiled because they're binary crates. The binary file format is documented in README.md and in the `stupid_binary` crate docs, so keep both in sync.

## Current state vs. design

The README documents the full intended instruction set (14 opcodes, `0000`–`1101`: `HALT`, `INIT`, `LOAD`, `SAVE`, `MOVE`, `WRITE`, `READ`, `ADD`, `AND`, `OR`, `NOT`, `JUMP`, `JPZR`, `JPOV`). Instructions are 16-bit words: a 4-bit opcode, a 2-bit register argument and a 10-bit value/address argument. Only `HALT` and `INIT` are implemented so far. Adding an instruction touches four places: the `Instruction` enum, `integer_to_inst`/`inst_to_integer` in `stupid_binary`, `parse_instruction` in `stupid_assembler`, and `Vm::execute`. Decoding any other opcode panics, and so does assembling any other operation.

The VM has 4 registers and 1024 memory cells (`REG_COUNT`/`MEMCELL_COUNT` in [vm.rs](crates/interpreter/src/vm.rs)), matching the README. When implementing new instructions, keep the README's instruction table and this file in sync with what the codec, the assembler and `Vm::execute` actually handle.

## Conventions

- Doc comments (`///` / `//!`) follow standard rustdoc conventions: short description, `# Parameters`/params via natural prose, `# Panics` for panicking behavior, `# Side Effects` where mutation isn't obvious from the signature.
  - Provide examples for public facing functions
- Leave [README.md](README.md) to the user to update. Don't work on it. Just update code docs
- Don't touch the `.bin` or `.notbin.txt` files (e.g. `example.bin`). Don't edit, regenerate or document them.
- Have fun with this. It's light hearted and meant to be humorous
