//! A quick-and-dirty loader for HEE HOO binary files.
//!
//! [`StupidBinary::load`] reads a program file into a [`StupidBinary`], which
//! implements [`Binary`] so the VM can run it. The file layout (all
//! multi-byte numbers little-endian) is:
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
//! When [`DEBUG_STUPIDLY`] is on, loading prints each header field and the
//! decoded sections to stdout.

use crate::binary::{Binary, Instruction};
use std::{fs::File, io::Read};

/// The magic text every HEE HOO binary must open with. Hello everybody.
const MAGIC_TEXT: &str = "markiplier";
/// Length of [`MAGIC_TEXT`] in bytes.
const MAGIC_WORD_LENGTH: usize = const { MAGIC_TEXT.len() };

/// When `true`, loading prints every header field, and [`decode_section`]
/// prints the raw bytes and decoded words of every non-empty section.
const DEBUG_STUPIDLY: bool = true;

/// Like `println!`, but only talks when [`DEBUG_STUPIDLY`] says it can.
macro_rules! stupid_println {
    ($($stuff:tt)*) => {
        if DEBUG_STUPIDLY {
            println!($($stuff)*);
        }
    };
}

/// A program loaded from a HEE HOO binary file, ready to be run by a
/// [`crate::vm::Vm`].
pub struct StupidBinary {
    /// The instruction sequence, indexed by instruction pointer.
    pub text: Vec<Instruction>,
    /// The instruction pointer value to start execution at.
    pub start_ptr: u16,
    /// Values to preload into data memory (starting at address `0`) before
    /// execution begins.
    pub initial_data: Vec<u16>,
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
    /// with an unimplemented opcode (see [`Instruction::parse_inst`]). The
    /// version is not validated. It's just vibes.
    ///
    /// # Examples
    /// ```no_run
    /// let binary = StupidBinary::load("heehoo.bin");
    /// let mut vm = Vm::default();
    /// vm.execute(binary);
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
                Instruction::parse_inst(address, value)
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

/// Decodes a byte buffer into little-endian 16-bit words.
///
/// # Parameters
/// - `data` (`&[u8]`): the raw bytes; must have an even length.
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
        print!(" ] ==> Words: [");
        for word in &words {
            print!(" {word:04X}");
        }
        println!(" ] ");
    };

    // Output words
    words
}
