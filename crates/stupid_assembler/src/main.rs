//! The stupid assembler: turns HEE HOO assembly into a HEE HOO binary.
//!
//! Reads `example.hasm` from the current working directory and writes the
//! assembled program to `example.bin` (via
//! [`stupid_binary::StupidBinary::to_bytes`]), ready for `interpreter` to
//! run. It narrates every line it parses, whether you asked or not.
//!
//! The syntax is one thing per line. A section header like `[TEXT]` picks
//! the section, and every line after it is an instruction: an operation
//! (case-insensitive) followed by its whitespace-separated arguments:
//!
//! ```text
//! [TEXT]
//! init 0 10
//! init 1 20
//! halt
//! ```
//!
//! Only the `[TEXT]` section and the `halt` and `init <reg> <val>`
//! instructions exist so far. Anything else (an unknown section or
//! operation, a missing or unparseable argument, or an instruction before
//! any section) panics.

use binary::Instruction;
use regex::regex;
use std::{
    fs::{self, read_to_string},
    io::Write,
};
use stupid_binary::StupidBinary;

#[derive(Debug)]
enum SectionType {
    Text,
}

impl SectionType {
    fn from_string(string: &str) -> Option<Self> {
        let pattern = regex!(r"\A\[[A-Z_0-9]+\]\z");
        let m = pattern.find(string)?;
        let sec = match m.as_str() {
            "[TEXT]" => Self::Text,
            _ => {
                panic!("Invalid section header: {string:?}");
            }
        };
        Some(sec)
    }
}

fn parse_instruction(string: &str) -> Instruction {
    let words: Vec<_> = string.split_whitespace().map(String::from).collect();
    let oper = words[0].to_lowercase();
    let arg1 = words.get(1);
    let arg2 = words.get(2);
    match oper.as_str() {
        "halt" => Instruction::Halt,
        "init" => {
            let reg = arg1
                .expect("Expected Argument 1")
                .parse()
                .expect("Can't parse Argument 1");
            let value = arg2
                .expect("Expected Argument 2")
                .parse()
                .expect("Can't parse Argument 2");
            Instruction::Init(reg, value)
        }
        _ => {
            panic!("Invalid instruction operation {}! What iz dis?", oper)
        }
    }
}

fn parse_text(text: &str) -> StupidBinary {
    let mut binary = StupidBinary::default();
    let mut current_section: Option<SectionType> = None;
    for line in text.lines() {
        println!("LINE: {line:?}");
        if let Some(section) = SectionType::from_string(line) {
            println!("  Parse section: {section:?}");
            current_section = Some(section);
        } else {
            match current_section {
                Some(SectionType::Text) => {
                    let inst = parse_instruction(line);
                    println!("  Parsed instruction: {inst:?}");
                    binary.text.push(inst);
                }
                None => panic!("No section was declared! Declare a text section with [TEXT]"),
            }
        }
    }
    binary
}

fn main() {
    let text = read_to_string("example.hasm").expect("where file");
    let binary = parse_text(&text);
    let binary_bytes = binary.to_bytes();
    println!("Binary size: {} bytes", binary_bytes.len());
    let mut binfile = fs::File::create("example.bin").expect("Oh noooes");
    binfile.write_all(&binary_bytes).expect("Mom I frew up");
}
