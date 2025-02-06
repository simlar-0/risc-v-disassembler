use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::helpers::extract_bits;

pub(crate) fn parse_btype32(opcode: &u8, imm: &i16, funct3: &u8, rs1: &u8, rs2: &u8) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}