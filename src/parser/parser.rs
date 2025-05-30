use crate::instructions::{DecodedInstruction32, ParseInstruction32, ParsedInstruction32};
use crate::parser::btype::parse_btype32;
use crate::parser::itype::parse_itype32;
use crate::parser::jtype::parse_jtype32;
use crate::parser::rtype::parse_rtype32;
use crate::parser::stype::parse_stype32;
use crate::parser::utype::parse_utype32;
use crate::{DisassemblerError, Register};

impl ParseInstruction32 for DecodedInstruction32 {
    fn parse_instruction32<T: Register>(&self) -> Result<ParsedInstruction32, DisassemblerError> {
        match self {
            DecodedInstruction32::RType {
                opcode,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => parse_rtype32::<T>(opcode, rd, funct3, rs1, rs2, funct7),
            DecodedInstruction32::IType {
                opcode,
                rd,
                funct3,
                rs1,
                imm,
            } => parse_itype32::<T>(opcode, rd, funct3, rs1, imm),
            DecodedInstruction32::SType {
                opcode,
                imm,
                funct3,
                rs1,
                rs2,
            } => parse_stype32::<T>(opcode, imm, funct3, rs1, rs2),
            DecodedInstruction32::BType {
                opcode,
                imm,
                funct3,
                rs1,
                rs2,
            } => parse_btype32::<T>(opcode, imm, funct3, rs1, rs2),
            DecodedInstruction32::UType { opcode, rd, imm } => parse_utype32::<T>(opcode, rd, imm),
            DecodedInstruction32::JType { opcode, rd, imm } => parse_jtype32::<T>(opcode, rd, imm),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::NumberedRegister;

    use super::*;

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
        let result = decoded.parse_instruction32::<NumberedRegister>().unwrap();
        assert!(matches!(result, ParsedInstruction32::add { .. }));
    }

    #[test]
    fn test_parse_instruction32_itype_addi() {
        let decoded = DecodedInstruction32::IType {
            opcode: 0b0010011,
            rd: 0b00001,
            funct3: 0b000,
            rs1: 0b00010,
            imm: 1,
        };
        let result = decoded.parse_instruction32::<NumberedRegister>().unwrap();
        assert!(matches!(result, ParsedInstruction32::addi { .. }));
    }

    #[test]
    fn test_parse_instruction32_stype_sb() {
        let decoded = DecodedInstruction32::SType {
            opcode: 0b0100011,
            imm: 1,
            funct3: 0b000,
            rs1: 0b00010,
            rs2: 0b00011,
        };
        let result = decoded.parse_instruction32::<NumberedRegister>().unwrap();
        assert!(matches!(result, ParsedInstruction32::sb { .. }));
    }

    #[test]
    fn test_parse_instruction32_btype_beq() {
        let decoded = DecodedInstruction32::BType {
            opcode: 0b1100011,
            imm: 1,
            funct3: 0b000,
            rs1: 0b00010,
            rs2: 0b00011,
        };
        let result = decoded.parse_instruction32::<NumberedRegister>().unwrap();
        assert!(matches!(result, ParsedInstruction32::beq { .. }));
    }

    #[test]
    fn test_parse_instruction32_utype_lui() {
        let decoded = DecodedInstruction32::UType {
            opcode: 0b0110111,
            rd: 0b00001,
            imm: (1 as i32) << 12,
        };
        let result = decoded.parse_instruction32::<NumberedRegister>().unwrap();
        assert!(matches!(result, ParsedInstruction32::lui { .. }));
    }

    #[test]
    fn test_parse_instruction32_jtype_jal() {
        let decoded = DecodedInstruction32::JType {
            opcode: 0b1101111,
            rd: 0b00001,
            imm: 1,
        };
        let result = decoded.parse_instruction32::<NumberedRegister>().unwrap();
        assert!(matches!(result, ParsedInstruction32::jal { .. }));
    }
}

