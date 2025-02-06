use crate::instructions::{DecodedInstruction32, ParseInstruction32, ParsedInstruction32};

impl ParseInstruction32 for DecodedInstruction32 {
    fn parse_instruction32(&self) -> Result<ParsedInstruction32, &'static str> {
        todo!()
    }
}