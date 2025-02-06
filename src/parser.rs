use crate::instructions::{DecodedInstruction32, ParseInstruction32, ParsedInstruction32};


impl ParseInstruction32 for DecodedInstruction32 {
    fn parse_instruction32(&self) -> Result<ParsedInstruction32, &'static str> {
        match self {
            DecodedInstruction32::RType { opcode, rd, funct3, rs1, rs2, funct7 } => 
                parse_rtype32(opcode, rd, funct3, rs1, rs2, funct7),
            DecodedInstruction32::IType { opcode, rd, funct3, rs1, imm } =>
                parse_itype32(opcode, rd, funct3, rs1, imm),
            DecodedInstruction32::SType { opcode, imm, funct3, rs1, rs2 } =>
                parse_stype32(opcode, imm, funct3, rs1, rs2),
            DecodedInstruction32::BType { opcode, imm, funct3, rs1, rs2 } =>
                parse_btype32(opcode, imm, funct3, rs1, rs2),
            DecodedInstruction32::UType { opcode, rd, imm } =>
                parse_utype32(opcode, rd, imm),
            DecodedInstruction32::JType { opcode, rd, imm } =>
                parse_jtype32(opcode, rd, imm),
        }
    }
}

fn parse_rtype32(opcode: &u8, rd: &u8, funct3: &u8, rs1: &u8, rs2: &u8, funct7: &u8) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

fn parse_itype32(opcode: &u8, rd: &u8, funct3: &u8, rs1: &u8, imm: &u16) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

fn parse_stype32(opcode: &u8, imm: &u16, funct3: &u8, rs1: &u8, rs2: &u8) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

fn parse_btype32(opcode: &u8, imm: &u16, funct3: &u8, rs1: &u8, rs2: &u8) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

fn parse_utype32(opcode: &u8, rd: &u8, imm: &u32) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

fn parse_jtype32(opcode: &u8, rd: &u8, imm: &u32) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}








