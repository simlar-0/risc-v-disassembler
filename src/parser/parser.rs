use crate::instructions::{DecodedInstruction32, ParseInstruction32, ParsedInstruction32};
use crate::parser::rtype::parse_rtype32;
use crate::parser::itype::parse_itype32;
use crate::parser::stype::parse_stype32;
use crate::parser::btype::parse_btype32;
use crate::parser::utype::parse_utype32;
use crate::parser::jtype::parse_jtype32;


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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::Register;

    #[test]
    fn test_parse_instruction32_rtype_add() {
        let decoded = DecodedInstruction32::RType {
            opcode: 0b0110011,
            rd: 0b00001,
            funct3: 0b000,
            rs1: 0b00010,
            rs2: 0b00011,
            funct7: 0b0000000,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert_eq!(result, ParsedInstruction32::add {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
        });
    }

    #[test]
    fn test_parse_instruction32_itype_addi() {
        let decoded = DecodedInstruction32::IType {
            opcode: 0b0010011,
            rd: 0b00001,
            funct3: 0b000,
            rs1: 0b00010,
            imm: 0b000000000001,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert_eq!(result, ParsedInstruction32::addi {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }
}