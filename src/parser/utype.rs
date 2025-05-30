use crate::instructions::{parsed_instructions::*, ParsedInstruction32};
use crate::registers::Register;
use crate::DisassemblerError;

pub(crate) fn parse_utype32<T: Register>(
    opcode: &u8,
    rd: &u8,
    imm: &i32,
) -> Result<ParsedInstruction32, DisassemblerError> {
    let rd = T::from_u8(*rd)?.as_string();

    match opcode {
        0b0110111 => Ok(ParsedInstruction32::lui(lui { rd, imm: *imm })),
        0b0010111 => Ok(ParsedInstruction32::auipc(auipc { rd, imm: *imm })),
        _ => Err(DisassemblerError::InvalidOpcode(*opcode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::ParsedInstruction32;
    use crate::registers::NumberedRegister;

    #[test]
    fn test_parse_utype32_lui() {
        let imm = 1 << 12;
        let result = parse_utype32::<NumberedRegister>(&0b0110111, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::lui { .. }));
    }

    #[test]
    fn test_parse_utype32_auipc() {
        let imm = 1 << 12;
        let result = parse_utype32::<NumberedRegister>(&0b0010111, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::auipc { .. }));
    }

    #[test]
    fn test_parse_utype32_invalid_opcode() {
        let imm = 1 << 12;
        let result = parse_utype32::<NumberedRegister>(&0b0000000, &0b00010, &imm);
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(DisassemblerError::InvalidOpcode(0b0000000))
        );
    }
}
