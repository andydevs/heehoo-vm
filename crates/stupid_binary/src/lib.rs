//! A quick-and-dirty reader and writer for HEE HOO binary files.
//!
//! [`StupidBinary::load`] reads a program file into a [`StupidBinary`], which
//! implements [`Binary`] (from `binary`) so the VM can run it.
//! [`StupidBinary::to_bytes`] goes the other way, turning a [`StupidBinary`]
//! back into file bytes. It always writes version `0` and start pointer `0`,
//! because ambition is overrated. This crate also converts instructions to
//! and from their 16-bit machine words (`oooo aa bbbbbbbbbb`: 4-bit opcode,
//! 2-bit first argument, 10-bit second argument).
//!
//! The file layout (all multi-byte numbers little-endian) is:
//!
//! | Offset | Size        | Field                                     |
//! | :----- | :---------- | :---------------------------------------- |
//! | 0      | 10          | Magic text: ASCII `markiplier` (checked)  |
//! | 10     | 1           | Version number (read but not checked)     |
//! | 11     | 2           | Instruction section size, in bytes        |
//! | 13     | 2           | Data section size, in bytes               |
//! | 15     | 2           | Start instruction pointer                 |
//! | 17     | inst size   | Instructions, one 16-bit word each        |
//! | ...    | data size   | Initial data memory, one 16-bit word each |
//!
//! When the `DEBUG_STUPIDLY` flag is on, loading prints each header field and
//! the decoded sections to stdout, and writing prints the encoded sections
//! and the final bytes.

use binary::{Binary, Instruction};
use std::{fs::File, io::Read};

/// When `true`, loading prints every header field, and [`decode_section`]
/// prints the raw bytes and decoded words of every non-empty section.
const DEBUG_STUPIDLY: bool = true;

/// The magic text every HEE HOO binary must open with. Hello everybody.
const MAGIC_TEXT: &str = "markiplier";
/// Length of [`MAGIC_TEXT`] in bytes.
const MAGIC_WORD_LENGTH: usize = const { MAGIC_TEXT.len() };

/// Bits of an instruction word holding the opcode (the top 4).
const OPCODE_MASK: u16 = 0b1111_0000_0000_0000;
/// How far to shift a masked opcode right to get its value.
const OPCODE_OFFSET: u8 = 12;

/// Bits of an instruction word holding the first (register) argument.
const ARG1_MASK: u16 = 0b0000_1100_0000_0000;
/// How far to shift a masked first argument right to get its value.
const ARG1_OFFSET: u8 = 10;

/// Bits of an instruction word holding the second (value/address) argument.
/// Already sitting at the bottom, so no shifting required. Lazy bits.
const ARG2_MASK: u16 = 0b0000_0011_1111_1111;

/// Opcode for [`Instruction::Halt`].
const OPCODE_HALT: u8 = 0b0000;
/// Opcode for [`Instruction::Init`].
const OPCODE_INIT: u8 = 0b0001;

/// Like `println!`, but only talks when [`DEBUG_STUPIDLY`] says it can.
macro_rules! stupid_println {
    ($($stuff:tt)*) => {
        if DEBUG_STUPIDLY {
            println!($($stuff)*);
        }
    };
}

/// A program loaded from a HEE HOO binary file, ready to be run by the
/// HEE HOO VM.
#[derive(Debug, Default)]
pub struct StupidBinary {
    /// The instruction sequence, indexed by instruction pointer.
    pub text: Vec<Instruction>,
    /// The instruction pointer value to start execution at.
    pub start_ptr: u16,
    /// Values to preload into data memory (starting at address `0`) before
    /// execution begins.
    pub initial_data: Vec<u16>,
}

impl StupidBinary {
    /// Convert the stupid binary into its stupid bytes
    ///
    /// # Returns
    /// The bytes n shiz
    pub fn to_bytes(&self) -> Vec<u8> {
        // Convert instruction list into int vector. Get section sizes as u16's
        let text: Vec<u16> = self.text.iter().map(|inst| inst_to_integer(*inst)).collect();
        let text_size: u16 = (self.text.len() * 2).try_into().expect("Text size too big");
        let initial_data_size: u16 = (self.initial_data.len() * 2).try_into().expect("Data size too big");

        // Build and concat list
        let bin_final = [
            // Markiplier
            MAGIC_TEXT.as_bytes(),
            // Version number
            &[0u8],
            // Text length in bytes
            &text_size.to_le_bytes(),
            // Data length in bytes
            &initial_data_size.to_le_bytes(),
            // Stupid binary always start at 0
            &[0u8, 0u8],
            // Text section
            &encode_section(&text),
            // Initial data section
            &encode_section(&self.initial_data),
        ]
        .concat();

        // Print it if we need to
        if DEBUG_STUPIDLY && !bin_final.is_empty() {
            print!("Final Binary: [");
            for byte in &bin_final {
                print!(" {byte:02X}");
            }
            print!(" ]");
            println!();
        }

        // Return the ting
        bin_final
    }
}

impl Binary for StupidBinary {
    /// Returns the start pointer read from the file header.
    ///
    /// # Returns
    /// [`StupidBinary::start_ptr`].
    fn init_start_ptr(&self) -> u16 {
        self.start_ptr
    }

    /// Returns the instruction at `address` in [`StupidBinary::text`].
    ///
    /// # Parameters
    /// - `address` (`u16`): index into `text`.
    ///
    /// # Returns
    /// A copy of the [`Instruction`] at `address`.
    ///
    /// # Panics
    /// Panics if `address` is past the end of `text`.
    fn fetch_instruction(&self, address: u16) -> Instruction {
        self.text[usize::from(address)]
    }

    /// Returns the data section read from the file.
    ///
    /// # Returns
    /// [`StupidBinary::initial_data`] as a slice.
    fn initial_data(&self) -> &[u16] {
        &self.initial_data
    }
}

impl StupidBinary {
    /// Loads a HEE HOO binary file into a [`StupidBinary`].
    ///
    /// See the [module docs](self) for the file layout.
    ///
    /// # Parameters
    /// - `path` (`&str`): path of the binary file to read, relative to the
    ///   current working directory unless absolute.
    ///
    /// # Returns
    /// A [`StupidBinary`] holding the decoded instructions, start pointer,
    /// and initial data.
    ///
    /// # Panics
    /// Panics if the file can't be opened, ends before a header field or
    /// section is fully read, has a magic text that isn't UTF-8 or isn't
    /// `markiplier`, has an odd-sized section, or contains an instruction
    /// with an unimplemented opcode (see `integer_to_inst`). The
    /// version is not validated. It's just vibes.
    ///
    /// # Examples
    /// ```no_run
    /// use binary::Binary;
    /// use stupid_binary::StupidBinary;
    ///
    /// let binary = StupidBinary::load("heehoo.bin");
    /// println!("Starting at {}, hold onto your butts", binary.init_start_ptr());
    /// ```
    pub fn load(path: &str) -> Self {
        let mut f = File::open(path).expect("where file?");
        let mut magic_buf = [0; MAGIC_WORD_LENGTH];
        let mut version_buf = [0; 1];
        let mut inst_size_buf = [0; 2];
        let mut data_size_buf = [0; 2];
        let mut start_ptr_buf = [0; 2];

        // Just "markiplier" in ascii
        f.read_exact(&mut magic_buf).expect("Der numbur didnt redded gud");
        let magic_str = str::from_utf8(&magic_buf).expect("Why");
        if magic_str != MAGIC_TEXT {
            panic!("What dis file?! This no say {MAGIC_TEXT:?} it say {magic_str:?}");
        }
        stupid_println!("Magic Text: {magic_str:?}");

        // Version shit
        f.read_exact(&mut version_buf).expect("Der numbur didnt redded gud");
        let _version = u8::from_le_bytes(version_buf);
        stupid_println!("Version Number: {_version:?}");

        // Instruction size
        f.read_exact(&mut inst_size_buf).expect("Der numbur didnt redded gud");
        let inst_size = u16::from_le_bytes(inst_size_buf);
        stupid_println!("Instruction Size Number: {inst_size:?}");

        // Data size
        f.read_exact(&mut data_size_buf).expect("Der numbur didnt redded gud");
        let data_size = u16::from_le_bytes(data_size_buf);
        stupid_println!("Data Size Number: {data_size:?}");

        // Instruction text start pointer
        f.read_exact(&mut start_ptr_buf).expect("Der numbur didnt redded gud");
        let start_ptr = u16::from_le_bytes(start_ptr_buf);
        stupid_println!("Start Pointer Number: {start_ptr:?}");

        // Instruction text
        let mut text_binary: Vec<u8> = vec![0; inst_size.into()];
        stupid_println!("Text binary size: {}", text_binary.len());
        f.read_exact(&mut text_binary).expect("Der numbur didnt redded gud");
        let text = decode_section(&text_binary)
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
                let address: u16 = index.try_into().expect("Why no number");
                integer_to_inst(address, value)
            })
            .collect();
        stupid_println!("Text: {text:?}");

        // Initial data text
        let mut initial_data_binary: Vec<u8> = vec![0; data_size.into()];
        f.read_exact(&mut initial_data_binary)
            .expect("Der numbur didnt redded gud");
        let initial_data = decode_section(&initial_data_binary);
        stupid_println!("Initial data: {initial_data:?}");

        Self {
            text,
            start_ptr,
            initial_data,
        }
    }
}

/// Decodes an instruction from its 16-bit machine-word encoding.
///
/// The word is split as `oooo aa bbbbbbbbbb` (most significant bit
/// first): a 4-bit opcode, a 2-bit first argument, and a 10-bit second
/// argument. Arguments an instruction doesn't use are ignored.
///
/// # Parameters
/// - `address` (`u16`): where this word lives in the instruction section.
///   Only used to point fingers in the panic message.
/// - `value` (`u16`): the encoded instruction word.
///
/// # Returns
/// The decoded [`Instruction`].
///
/// # Panics
/// Panics if the opcode doesn't belong to an implemented instruction
/// (currently anything other than `HALT` or `INIT`)
fn integer_to_inst(address: u16, value: u16) -> Instruction {
    // Parse instruction opcode
    let opcode: u8 = ((value & OPCODE_MASK) >> OPCODE_OFFSET)
        .try_into()
        .expect("Could not convert opcode u16 to u8");

    // Parse instruction argument 1
    let arg1: u8 = ((value & ARG1_MASK) >> ARG1_OFFSET)
        .try_into()
        .expect("Could not convert instruction arg 1 u16 to u8");

    // Parse instruction argument 2
    let arg2: u16 = value & ARG2_MASK;

    // Match opcode
    match opcode {
        OPCODE_HALT => Instruction::Halt,
        OPCODE_INIT => Instruction::Init(arg1, arg2),
        _ => panic!("Invalid opcode: {address}: {opcode}"),
    }
}

/// Convert instruction into a u16 integer
///
/// # Parameters
/// - `inst`: the instruction enum to encode
///
/// # Returns
/// The `u16` equivalent of the given instruction
fn inst_to_integer(inst: Instruction) -> u16 {
    match inst {
        Instruction::Halt => OPCODE_HALT.into(),
        Instruction::Init(reg, val) => {
            let opcode: u16 = u16::from(OPCODE_INIT) << OPCODE_OFFSET;
            let reg: u16 = u16::from(reg) << ARG1_OFFSET;
            opcode | reg | val
        }
    }
}

/// Decodes a byte buffer into little-endian 16-bit words.
///
/// # Parameters
/// - `data`: the raw bytes; must have an even length.
///
/// # Returns
/// A `Vec<u16>` with one word per pair of bytes, in order.
///
/// # Panics
/// Panics if `data` has an odd length.
fn decode_section(data: &[u8]) -> Vec<u16> {
    // Parse data into 2-byte chunks
    let (chunks, []) = data.as_chunks::<2>() else {
        panic!("Data buffer is not aligned for 16 bit values! You dolt");
    };

    // Convert chunks into 16-bit words little-endian encoded
    let words: Vec<_> = chunks.iter().map(|chunk| u16::from_le_bytes(*chunk)).collect();

    // Debug output if enabled
    if DEBUG_STUPIDLY && !(data.is_empty() && words.is_empty()) {
        print!("Bytes: [");
        for byte in data {
            print!(" {byte:02X}");
        }
        print!(" ]");
        print!(" ==> ");
        print!("Words: [");
        for word in &words {
            print!(" {word:04X}");
        }
        println!(" ] ");
    };

    // Output words
    words
}

/// Encode data section into bytes
///
/// # Parameters
/// - `data`: Slice of `u16` values
///
/// # Returns
/// Data encoded into bytes
///
/// # Side Effects
/// If `DEBUG_STUPIDLY` is enabled, it yaps every buffer it gets
fn encode_section(data: &[u16]) -> Vec<u8> {
    // Convert bytes
    let bytes = data.iter().flat_map(|n| n.to_le_bytes()).collect();

    // Debug output if enabled
    if DEBUG_STUPIDLY && !(data.is_empty() && data.is_empty()) {
        print!("Words: [");
        for word in data {
            print!(" {word:04X}");
        }
        print!(" ]");
        print!(" ==> ");
        print!("Bytes: [");
        for byte in &bytes {
            print!(" {byte:02X}");
        }
        println!(" ] ");
    };

    // Output bytes
    bytes
}
