use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::helpers::variable_bit_structures::VarBitInt;

pub(crate) fn parse_jtype32(opcode: &u8, rd: &u8, imm: &VarBitInt) -> Result<ParsedInstruction32, &'static str> {
    let imm = i32::try_from(*imm)?;
    match opcode {
        0b1101111 => Ok(ParsedInstruction32::jal {
            rd: Register::try_from(*rd)?,
            imm: imm,
        }),
        _ => Err("Invalid opcode"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::ParsedInstruction32;

    #[test]
    fn test_parse_jtype32_jal() {
        let imm = VarBitInt::new(0b00000000000000000000000000000001, 20);
        let result = parse_jtype32(&0b1101111, &0b00010, &imm).unwrap();
        assert!(matches!(result, ParsedInstruction32::jal {..}));
    }

    #[test]
    fn test_parse_jtype32_invalid_opcode() {
        let imm = VarBitInt::new(0b00000000000000000000000000000001, 20);
        let result = parse_jtype32(&0b0000000, &0b00010, &imm);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid opcode"));
    }
}