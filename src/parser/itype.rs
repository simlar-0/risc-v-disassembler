use crate::helpers::variable_bit_structures::VarBitInt;
use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::macros::extract_bits;
use crate::DisassemblerError;

pub(crate) fn parse_itype32(opcode: &u8, rd: &u8, funct3: &u8, rs1: &u8, imm: &VarBitInt) -> Result<ParsedInstruction32, DisassemblerError> {
    let imm = i32::try_from(*imm)?;
    let rd = Register::try_from(*rd)?;
    let rs1 = Register::try_from(*rs1)?;
    
    match opcode {
        0b0000011 => parse_itype32_load(funct3, rd, rs1, imm),
        0b0010011 => parse_itype32_alu(funct3, rd, rs1, imm),
        0b1100111 => Ok(ParsedInstruction32::jalr { rd, rs1, imm }),
        0b1110011 => {
            match imm {
                0b000000000000 => Ok(ParsedInstruction32::ecall),
                0b000000000001 => Ok(ParsedInstruction32::ebreak),
                _ => Err(DisassemblerError::InvalidImmediate(imm)),
            }
        },
        _ => Err(DisassemblerError::InvalidOpcode(*opcode)),
    }
}

fn parse_itype32_load(funct3: &u8, rd: Register, rs1: Register, imm: i32) -> Result<ParsedInstruction32, DisassemblerError> {
    match funct3 {
        0b000 => Ok(ParsedInstruction32::lb { rd, rs1, imm }),
        0b001 => Ok(ParsedInstruction32::lh { rd, rs1, imm }),
        0b010 => Ok(ParsedInstruction32::lw { rd, rs1, imm }),
        0b100 => Ok(ParsedInstruction32::lbu { rd, rs1, imm }),
        0b101 => Ok(ParsedInstruction32::lhu { rd, rs1, imm }),
        _ => Err(DisassemblerError::InvalidFunct3(*funct3)),
    }
}

fn parse_itype32_alu(funct3: &u8, rd: Register, rs1: Register, imm: i32) -> Result<ParsedInstruction32, DisassemblerError> {
    let imm_upper_bits = extract_bits!(imm, 5, 11)?;
    let shamt = extract_bits!(imm, 0, 4)? as u8;

    match funct3 {
        0b000 => Ok(ParsedInstruction32::addi { rd, rs1, imm }),
        0b001 => Ok(ParsedInstruction32::slli { rd, rs1, shamt }),
        0b010 => Ok(ParsedInstruction32::slti { rd, rs1, imm }),
        0b011 => Ok(ParsedInstruction32::sltiu { rd, rs1, imm }),
        0b100 => Ok(ParsedInstruction32::xori { rd, rs1, imm }),
        0b101 => match imm_upper_bits {
            0b0000000 => Ok(ParsedInstruction32::srli { rd, rs1, shamt }),
            0b0100000 => Ok(ParsedInstruction32::srai { rd, rs1, shamt }),
            _ => Err(DisassemblerError::InvalidImmediate(imm)),
        },
        0b110 => Ok(ParsedInstruction32::ori { rd, rs1, imm }),
        0b111 => Ok(ParsedInstruction32::andi { rd, rs1, imm }),
        _ => Err(DisassemblerError::InvalidFunct3(*funct3)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_itype32_lb() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0000011, &0b00001, &0b000, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::lb {..}));
    }

    #[test]
    fn test_parse_itype32_lh() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0000011, &0b00001, &0b001, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::lh {..}));
    }

    #[test]
    fn test_parse_itype32_lw() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0000011, &0b00001, &0b010, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::lw {..}));
    }

    #[test]
    fn test_parse_itype32_lbu() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0000011, &0b00001, &0b100, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::lbu {..}));
    }

    #[test]
    fn test_parse_itype32_lhu() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0000011, &0b00001, &0b101, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::lhu {..}));
    }

    #[test]
    fn test_parse_itype32_load_invalid_funct3() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0000011, &0b00001, &0b110, &0b00010, &imm);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidFunct3(0b110)));
    }

    #[test]
    fn test_parse_itype32_addi() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b000, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::addi {..}));
    }

    #[test]
    fn test_parse_itype32_slli() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b001, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::slli {..}));
    }

    #[test]
    fn test_parse_itype32_slti() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b010, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::slti {..}));
    }

    #[test]
    fn test_parse_itype32_sltiu() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b011, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::sltiu {..}));
    }

    #[test]
    fn test_parse_itype32_xori() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b100, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::xori {..}));
    }

    #[test]
    fn test_parse_itype32_srli() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b101, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::srli {..}));
    }

    #[test]
    fn test_parse_itype32_srai() {
        let imm = VarBitInt::new(0b010000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b101, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::srai {..}));
    }

    #[test]
    fn test_parse_itype32_ori() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b110, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::ori {..}));
    }

    #[test]
    fn test_parse_itype32_andi() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b111, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::andi {..}));
    }

    #[test]
    fn test_parse_itype32_invalid_funct3() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b0010011, &0b00001, &0b1000, &0b00010, &imm);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidFunct3(0b1000)));
    }

    #[test]
    fn test_parse_itype32_invalid_imm() {
        let imm = VarBitInt::new(0b100000000001, 12);
        let exp_imm:i32 = -2047;
        let result = parse_itype32(&0b0010011, &0b00001, &0b101, &0b00010, &imm);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidImmediate(exp_imm)));
    }

    #[test]
    fn test_parse_itype32_jalr() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b1100111, &0b00001, &0b000, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::jalr {..}));
    }

    #[test]
    fn test_parse_itype32_ecall() {
        let imm = VarBitInt::new(0b000000000000, 12);
        let result = parse_itype32(&0b1110011, &0b00000, &0b000, &0b00000, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::ecall));
    }

    #[test]
    fn test_parse_itype32_ebreak() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b1110011, &0b00000, &0b000, &0b00000, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::ebreak));
    }

    #[test]
    fn test_parse_itype32_invalid_opcode() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_itype32(&0b1111111, &0b00001, &0b000, &0b00010, &imm);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidOpcode(0b1111111)));
    }

    #[test]
    fn test_parse_itype32_invalid_funct3_ecall_ebreak() {
        let imm = VarBitInt::new(0b000000000010, 12);
        let result = parse_itype32(&0b1110011, &0b00000, &0b000, &0b00000, &imm);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidImmediate(0b000000000010)));
    }
}