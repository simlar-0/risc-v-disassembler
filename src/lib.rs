use thiserror::Error;

mod helpers;
mod macros;
mod registers;
mod instructions;
mod decoder;
mod parser;

#[derive(Debug, Error, PartialEq)]
pub enum DisassemblerError {
    #[error("Invalid field: {1} with value {0:b}. The field value is not valid for the given instruction.")]
    InvalidField(u8, &'static str),
    
    #[error("Invalid register: {0:?}. The register index is out of bounds.")]
    InvalidRegister(u8),
    
    #[error("Invalid immediate: {0:b}. The immediate value is not valid for the given instruction.")]
    InvalidImmediate(i32),
    
    #[error("Bit extraction error: {0}.")]
    BitExtractionError(&'static str),
    
    #[error("Cannot convert a VarBit with size greater than {0} bits to {1}.")]
    VarBitSizeExceeded(u8, &'static str),
}

