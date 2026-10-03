//! A quick-and-dirty loader for HEE HOO binary files.
//!
//! [`load_binary_stupidly`] reads a program file into a [`Binary`]. The file
//! layout (all multi-byte numbers little-endian) is:
//!
//! | Offset | Size        | Field                                     |
//! | :----- | :---------- | :---------------------------------------- |
//! | 0      | 10          | Magic text: ASCII `markiplier`            |
//! | 10     | 1           | Version number (read but not checked)     |
//! | 11     | 2           | Instruction section size, in bytes        |
//! | 13     | 2           | Data section size, in bytes               |
//! | 15     | 2           | Start instruction pointer                 |
//! | 17     | inst size   | Instructions, one 16-bit word each        |
//! | ...    | data size   | Initial data memory, one 16-bit word each |
//!
//! Loading prints each header field and the decoded sections to stdout.

use crate::binary::{Binary, Instruction};
use std::{fs::File, io::Read};

/// When `true`, [`deserialize_data`] prints the raw bytes and decoded
/// words of every non-empty section it decodes.
const DEBUG_STUPIDLY: bool = true;

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
fn deserialize_data(data: &[u8]) -> Vec<u16> {
    let (chunks, []) = data.as_chunks::<2>() else {
        panic!("Data buffer is not aligned for 16 bit values! You dolt");
    };
    let words: Vec<_> = chunks.iter().map(|chunk| u16::from_le_bytes(*chunk)).collect();
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
    words
}

/// Loads a HEE HOO binary file into a [`Binary`].
///
/// See the [module docs](self) for the file layout.
///
/// # Parameters
/// - `path` (`&str`): path of the binary file to read, relative to the
///   current working directory unless absolute.
///
/// # Returns
/// A [`Binary`] holding the decoded instructions, start pointer, and
/// initial data.
///
/// # Panics
/// Panics if the file can't be opened, ends before a header field or section
/// is fully read, has a non-UTF-8 magic text, has an odd-sized section, or
/// contains an instruction with an unimplemented opcode (see
/// [`Instruction::from_u16`]). The magic text and version are not validated.
pub fn load_binary_stupidly(path: &str) -> Binary {
    let mut f = File::open(path).expect("where file?");
    let mut magic_buf = [0; 10];
    let mut version_buf = [0; 1];
    let mut inst_size_buf = [0; 2];
    let mut data_size_buf = [0; 2];
    let mut start_ptr_buf = [0; 2];

    // Just "markiplier" in ascii
    f.read_exact(&mut magic_buf).expect("Der numbur didnt redded gud");
    let magic_str = str::from_utf8(&magic_buf).expect("Why");
    println!("Magic Text: {magic_str:?}");

    // Version shit
    f.read_exact(&mut version_buf).expect("Der numbur didnt redded gud");
    let _version = u8::from_le_bytes(version_buf);
    println!("Version Number: {_version:?}");

    f.read_exact(&mut inst_size_buf).expect("Der numbur didnt redded gud");
    let inst_size = u16::from_le_bytes(inst_size_buf);
    println!("Instruction Size Number: {inst_size:?}");

    f.read_exact(&mut data_size_buf).expect("Der numbur didnt redded gud");
    let data_size = u16::from_le_bytes(data_size_buf);
    println!("Data Size Number: {data_size:?}");

    f.read_exact(&mut start_ptr_buf).expect("Der numbur didnt redded gud");
    let start_ptr = u16::from_le_bytes(start_ptr_buf);
    println!("Start Pointer Number: {start_ptr:?}");

    let mut text_binary: Vec<u8> = vec![0; inst_size.into()];
    println!("Text binary size: {}", text_binary.len());
    f.read_exact(&mut text_binary).expect("Der numbur didnt redded gud");
    let text = deserialize_data(&text_binary)
        .into_iter()
        .map(Instruction::from_u16)
        .collect();
    println!("Text: {text:?}");

    let mut initial_data_binary: Vec<u8> = vec![0; data_size.into()];
    f.read_exact(&mut initial_data_binary)
        .expect("Der numbur didnt redded gud");
    let initial_data = deserialize_data(&initial_data_binary);
    println!("Initial data: {initial_data:?}");

    Binary {
        text,
        start_ptr,
        initial_data,
    }
}
