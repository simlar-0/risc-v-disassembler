use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::helpers::variable_bit_structures::VarBitInt;
use crate::DisassemblerError;

pub(crate) fn parse_utype32(opcode: &u8, rd: &u8, imm: &VarBitInt) -> Result<ParsedInstruction32, DisassemblerError> {
    let imm = i32::try_from(*imm)?;
    let rd = Register::try_from(*rd)?;
    
    match opcode {
        0b0110111 => Ok(ParsedInstruction32::lui { rd, imm }),
        0b0010111 => Ok(ParsedInstruction32::auipc { rd, imm }),
        _ => Err(DisassemblerError::InvalidOpcode(*opcode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::ParsedInstruction32;

    #[test]
    fn test_parse_utype32_lui() {
        let imm = VarBitInt::new(0b00000000000000000000000000000001, 20);
        let result = parse_utype32(&0b0110111, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::lui {..}));
    }

    #[test]
    fn test_parse_utype32_auipc() {
        let imm = VarBitInt::new(0b00000000000000000000000000000001, 20);
        let result = parse_utype32(&0b0010111, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::auipc {..}));
    }

    #[test]
    fn test_parse_utype32_invalid_opcode() {
        let imm = VarBitInt::new(0b00000000000000000000000000000001, 20);
        let result = parse_utype32(&0b0000000, &0b00010, &imm);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidOpcode(0b0000000)));
    }
}