use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::helpers::extract_bits;

pub(crate) fn parse_jtype32(opcode: &u8, rd: &u8, imm: &i32) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}