use registers::Register;
use thiserror::Error;

mod helpers;
mod macros;
mod registers;
mod instructions;
mod decoder;
mod parser;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u8),
    #[error("Invalid funct3: {0}")]
    InvalidFunct3(u8),
    #[error("Invalid funct7: {0}")]
    InvalidFunct7(u8),
    #[error("Invalid register: {0:?}")]
    InvalidRegister(Register),
    #[error("Invalid immediate: {0}")]
    InvalidImmediate(u32),
    #[error("Bit extraction error: {0}")]
    BitExtractionError(String)
}

