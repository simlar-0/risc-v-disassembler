use crate::instructions::{DecodedInstruction32, ParseInstruction32, ParsedInstruction32};
use crate::Error;

impl ParseInstruction32 for DecodedInstruction32 {
    fn parse_instruction32(&self) -> Result<ParsedInstruction32, Error> {
        todo!()
    }
}