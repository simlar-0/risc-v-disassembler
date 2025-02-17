use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::DisassemblerError;

pub(crate) fn parse_rtype32(_opcode: &u8, rd: &u8, funct3: &u8, rs1: &u8, rs2: &u8, funct7: &u8) -> Result<ParsedInstruction32, DisassemblerError> {
    let rd = Register::try_from(*rd)?;
    let rs1 = Register::try_from(*rs1)?;
    let rs2 = Register::try_from(*rs2)?;
    
    match funct3 {
        0b000 => match funct7 {
            0b0000000 => Ok(ParsedInstruction32::add { rd, rs1, rs2 }),
            0b0100000 => Ok(ParsedInstruction32::sub { rd, rs1, rs2 }),
            _ => Err(DisassemblerError::InvalidFunct7(*funct7)),
        },
        0b001 => Ok(ParsedInstruction32::sll { rd, rs1, rs2 }),
        0b010 => Ok(ParsedInstruction32::slt { rd, rs1, rs2 }),
        0b011 => Ok(ParsedInstruction32::sltu { rd, rs1, rs2 }),
        0b100 => Ok(ParsedInstruction32::xor { rd, rs1, rs2 }),
        0b101 => match funct7 {
            0b0000000 => Ok(ParsedInstruction32::srl { rd, rs1, rs2 }),
            0b0100000 => Ok(ParsedInstruction32::sra { rd, rs1, rs2 }),
            _ => Err(DisassemblerError::InvalidFunct7(*funct7)),
        },
        0b110 => Ok(ParsedInstruction32::or { rd, rs1, rs2 }),
        0b111 => Ok(ParsedInstruction32::and { rd, rs1, rs2 }),
        _ => Err(DisassemblerError::InvalidFunct3(*funct3)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rtype32_add() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b000, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert!(matches!(result, ParsedInstruction32::add {..}));
    }

    #[test]
    fn test_parse_rtype32_sub() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b000, &0b00000010, &0b00000011, &0b0100000).unwrap();
        assert!(matches!(result, ParsedInstruction32::sub {..}));
    }

    #[test]
    fn test_parse_rtype32_sll() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b001, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert!(matches!(result, ParsedInstruction32::sll {..}));
    }

    #[test]
    fn test_parse_rtype32_slt() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b010, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert!(matches!(result, ParsedInstruction32::slt {..}));
    }

    #[test]
    fn test_parse_rtype32_sltu() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b011, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert!(matches!(result, ParsedInstruction32::sltu {..}));
    }

    #[test]
    fn test_parse_rtype32_xor() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b100, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert!(matches!(result, ParsedInstruction32::xor {..}));
    }

    #[test]
    fn test_parse_rtype32_srl() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b101, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert!(matches!(result, ParsedInstruction32::srl {..}));
    }

    #[test]
    fn test_parse_rtype32_sra() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b101, &0b00000010, &0b00000011, &0b0100000).unwrap();
        assert!(matches!(result, ParsedInstruction32::sra {..}));
    }

    #[test]
    fn test_parse_rtype32_or() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b110, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert!(matches!(result, ParsedInstruction32::or {..}));
    }

    #[test]
    fn test_parse_rtype32_and() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b111, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert!(matches!(result, ParsedInstruction32::and {..}));
    }

    #[test]
    fn test_parse_rtype32_invalid_funct3() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b1000, &0b00000010, &0b00000011, &0b0000000);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidFunct3(0b1000)));
    }

    #[test]
    fn test_parse_rtype32_invalid_funct7() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b000, &0b00000010, &0b00000011, &0b1000000);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidFunct7(0b1000000)));
    }

    #[test]
    fn test_parse_rtype32_invalid_funct7_for_funct3_101() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b101, &0b00000010, &0b00000011, &0b0010000);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidFunct7(0b0010000)));
    }
}