use crate::instructions::{DecodedInstruction32, ParseInstruction32, ParsedInstruction32};
use crate::DisassemblerError;
use crate::parser::rtype::parse_rtype32;
use crate::parser::itype::parse_itype32;
use crate::parser::stype::parse_stype32;
use crate::parser::btype::parse_btype32;
use crate::parser::utype::parse_utype32;
use crate::parser::jtype::parse_jtype32;


impl ParseInstruction32 for DecodedInstruction32 {
    fn parse_instruction32(&self) -> Result<ParsedInstruction32, DisassemblerError> {
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
    use crate::helpers::variable_bit_structures::VarBitInt;


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
        assert!(matches!(result, ParsedInstruction32::add {..}));
    }

    #[test]
    fn test_parse_instruction32_itype_addi() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let decoded = DecodedInstruction32::IType {
            opcode: 0b0010011,
            rd: 0b00001,
            funct3: 0b000,
            rs1: 0b00010,
            imm: imm,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert!(matches!(result, ParsedInstruction32::addi {..}));
    }

    #[test]
    fn test_parse_instruction32_stype_sb() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let decoded = DecodedInstruction32::SType {
            opcode: 0b0100011,
            imm: imm,
            funct3: 0b000,
            rs1: 0b00010,
            rs2: 0b00011,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert!(matches!(result, ParsedInstruction32::sb {..}));
    }

    #[test]
    fn test_parse_instruction32_btype_beq() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let decoded = DecodedInstruction32::BType {
            opcode: 0b1100011,
            imm: imm,
            funct3: 0b000,
            rs1: 0b00010,
            rs2: 0b00011,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert!(matches!(result, ParsedInstruction32::beq {..}));
    }

    #[test]
    fn test_parse_instruction32_utype_lui() {
        let imm = VarBitInt::new(0b000000000001, 20);
        let decoded = DecodedInstruction32::UType {
            opcode: 0b0110111,
            rd: 0b00001,
            imm: imm,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert!(matches!(result, ParsedInstruction32::lui {..}));
    }

    #[test]
    fn test_parse_instruction32_jtype_jal() {
        let imm = VarBitInt::new(0b000000000001, 20);
        let decoded = DecodedInstruction32::JType {
            opcode: 0b1101111,
            rd: 0b00001,
            imm: imm,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert!(matches!(result, ParsedInstruction32::jal {..}));
    }
}