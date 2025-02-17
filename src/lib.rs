//! A simple disassembler for the RISC-V instruction set architecture.
//! It currently supports RV32I instructions only.
//! 
//! The disassembler takes a byte array of 4 bytes and returns a parsed 
//! instruction in the form of a ParsedInstruction32 enum.
//! 
//! # Example
//! 
//! ```
//! use risc_v_disassembler::{parse, ParsedInstruction32, Register};
//! 
//! let bytes = [0x93, 0x00, 0x51, 0x00];
//! let parsed_instruction = parse(&bytes).unwrap();
//! 
//! assert_eq!(parsed_instruction, ParsedInstruction32::addi {
//!     rd: Register::x1,
//!     rs1: Register::x2,
//!     imm: 5
//! });
//! ```
//! 

mod registers;
mod instructions;
mod helpers;
mod macros;
mod decoder;
mod parser;

pub use instructions::ParsedInstruction32;
pub use registers::Register;
use instructions::{Instruction32, ParseInstruction32, DecodeInstruction32};
use thiserror::Error;

pub fn parse(bytes : &[u8]) -> Result<ParsedInstruction32, DisassemblerError> {
    if bytes.len() != 4 {
        return Err(DisassemblerError::InvalidInputLength(bytes.len()));
    }
    
    let instruction = Instruction32::from_le_bytes(bytes.try_into().unwrap());
    let parsed_instruction = instruction
        .decode_instruction32()?
        .parse_instruction32()?;
    Ok(parsed_instruction)
}

#[derive(Debug, Error, PartialEq)]
pub enum DisassemblerError {
    #[error("Invalid input length: {0}. The input must be 32 bits long.")]
    InvalidInputLength(usize),

    #[error("Invalid funct3 field with value {0:b}. The value is not valid for the given instruction.")]
    InvalidFunct3(u8),

    #[error("Invalid funct7 field with value {0:b}. The value is not valid for the given instruction.")]
    InvalidFunct7(u8),

    #[error("Invalid opcode field with value {0:b}. The value is not valid for the given instruction.")]
    InvalidOpcode(u8),
    
    #[error("Invalid immediate: {0:b}. The immediate value is not valid for the given instruction.")]
    InvalidImmediate(i32),

    #[error("Invalid register: {0:?}. The register index is out of bounds.")]
    InvalidRegister(u8),
    
    #[error("Bit extraction error: {0}.")]
    BitExtractionError(&'static str),
    
    #[error("Cannot convert a VarBit with size greater than {0} bits to {1}.")]
    VarBitSizeExceeded(u8, &'static str),
}

