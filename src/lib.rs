//! A simple disassembler for the RISC-V instruction set architecture.
//! It currently only supports 32 bit RV32I instructions.
//!
//! ### Supported Instruction Sets
//!  - RV32I
//!
//! Parses a byte array slice into a `ParsedInstruction32` enum representing a RISC-V instruction.
//!
//! ### Arguments
//!
//! * `bytes` - A slice of bytes representing the encoded instruction to be parsed.
//! * `is_big_endian` - A boolean indicating whether the bytes are in big endian format.
//!
//! ### Returns
//!
//! * `Ok(ParsedInstruction32)` - If the parsing is successful, returns the parsed instruction.
//! * `Err(DisassemblerError)` - If the parsing fails, returns an error indicating the reason for failure.
//!
//! ### Example
//!
//! ```
//! use risc_v_disassembler::{
//!     parse,
//!     ParsedInstruction32,
//!     parsed_instructions::*
//! };
//! use risc_v_disassembler::Register;
//!
//! let bytes = [0x93, 0x00, 0x51, 0x00];
//! let is_big_endian = false;
//! let use_abi_register_names = false;
//! let parsed_instruction = parse(&bytes, is_big_endian, use_abi_register_names).unwrap();
//!
//! assert_eq!(parsed_instruction, ParsedInstruction32::addi (addi {
//!     rd: "x1".to_string(),
//!     rs1: "x2".to_string(),
//!     imm: 5
//! }));
//! ```
//!

mod decoder;
mod instructions;
mod macros;
mod parser;
mod registers;

pub use instructions::{parsed_instructions, ParsedInstruction32};
use instructions::{DecodeInstruction32, Instruction32, ParseInstruction32};
pub use registers::Register;
use thiserror::Error;

pub fn parse(
    bytes: &[u8],
    is_big_endian: bool,
    use_abi_register_names: bool,
) -> Result<ParsedInstruction32, DisassemblerError> {
    if bytes.len() != 4 {
        return Err(DisassemblerError::UnsupportedInstructionLength(bytes.len()));
    }

    let instruction = if is_big_endian {
        Instruction32::from_be_bytes(bytes.try_into().unwrap())
    } else {
        Instruction32::from_le_bytes(bytes.try_into().unwrap())
    };

    let decoded_instruction = instruction.decode_instruction32()?;

    let parsed_instruction = if use_abi_register_names {
        decoded_instruction.parse_instruction32::<registers::ABIRegister>()?
    } else {
        decoded_instruction.parse_instruction32::<registers::NumberedRegister>()?
    };

    Ok(parsed_instruction)
}

#[derive(Debug, Error, PartialEq)]
pub enum DisassemblerError {
    #[error(
        "Unsupported instruction length: {0}. The length of the instruction is not supported."
    )]
    UnsupportedInstructionLength(usize),

    #[error(
        "Invalid funct3 field with value {0:b}. The value is not valid for the given instruction."
    )]
    InvalidFunct3(u8),

    #[error(
        "Invalid funct7 field with value {0:b}. The value is not valid for the given instruction."
    )]
    InvalidFunct7(u8),

    #[error(
        "Invalid opcode field with value {0:b}. The value is not valid for the given instruction."
    )]
    InvalidOpcode(u8),

    #[error(
        "Invalid immediate: {0:b}. The immediate value is not valid for the given instruction."
    )]
    InvalidImmediate(i32),

    #[error("Invalid register: {0:?}. The register index is out of bounds.")]
    InvalidRegister(u8),

    #[error("Bit extraction error: {0}.")]
    BitExtractionError(&'static str),

    #[error("Bit extension error: {0}.")]
    BitExtensionError(&'static str),
}
