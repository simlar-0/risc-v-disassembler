use crate::helpers::variable_bit_structures::VarBitInt;
use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::DisassemblerError;

pub(crate) fn parse_btype32(_opcode: &u8, imm: &VarBitInt, funct3: &u8, rs1: &u8, rs2: &u8) -> Result<ParsedInstruction32, DisassemblerError> {
    let rs1 = Register::try_from(*rs1)?;
    let rs2 = Register::try_from(*rs2)?;
    let imm = i32::try_from(*imm)?;
    
    match funct3 {
        0b000 => Ok(ParsedInstruction32::beq { rs1, rs2, imm }),
        0b001 => Ok(ParsedInstruction32::bne { rs1, rs2, imm }),
        0b100 => Ok(ParsedInstruction32::blt { rs1, rs2, imm }),
        0b101 => Ok(ParsedInstruction32::bge { rs1, rs2, imm }),
        0b110 => Ok(ParsedInstruction32::bltu { rs1, rs2, imm }),
        0b111 => Ok(ParsedInstruction32::bgeu { rs1, rs2, imm }),
        _ => Err(DisassemblerError::InvalidField(*funct3, "funct3")),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::ParsedInstruction32;

    #[test]
    fn test_parse_btype32_beq() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_btype32(&0b1100011, &imm, &0b000, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::beq {..}));
    }

    #[test]
    fn test_parse_btype32_bne() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_btype32(&0b1100011, &imm, &0b001, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::bne {..}));
    }

    #[test]
    fn test_parse_btype32_invalid_funct3() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_btype32(&0b1100011, &imm, &0b010, &0b00010, &0b00011);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidField(0b010, "funct3")));
    }

    #[test]
    fn test_parse_btype32_blt() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_btype32(&0b1100011, &imm, &0b100, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::blt {..}));
    }

    #[test]
    fn test_parse_btype32_bge() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_btype32(&0b1100011, &imm, &0b101, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::bge {..}));
    }

    #[test]
    fn test_parse_btype32_bltu() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_btype32(&0b1100011, &imm, &0b110, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::bltu {..}));
    }

    #[test]
    fn test_parse_btype32_bgeu() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_btype32(&0b1100011, &imm, &0b111, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::bgeu {..}));
    }
}
