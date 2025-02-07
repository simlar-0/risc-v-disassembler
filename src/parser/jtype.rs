use crate::instructions::ParsedInstruction32;
use crate::registers::Register;

pub(crate) fn parse_jtype32(opcode: &u8, rd: &u8, imm: &u32) -> Result<ParsedInstruction32, &'static str> {
    match opcode {
        0b1101111 => Ok(ParsedInstruction32::jal {
            rd: Register::try_from(*rd)?,
            imm: *imm as i32,
        }),
        _ => Err("Invalid opcode"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::ParsedInstruction32;
    use crate::registers::Register;

    #[test]
    fn test_parse_jtype32_jal() {
        let result = parse_jtype32(&0b1101111, &0b00010, &0b00000000000000000000000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::jal {
            rd: Register::try_from(0b00010).unwrap(),
            imm: 0b00000000000000000000000000000001 as i32,
        });
    }

    #[test]
    fn test_parse_jtype32_invalid_opcode() {
        let result = parse_jtype32(&0b0000000, &0b00010, &0b00000000000000000000000000000001);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid opcode"));
    }
}