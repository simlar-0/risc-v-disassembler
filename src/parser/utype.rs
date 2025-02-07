use crate::instructions::ParsedInstruction32;
use crate::registers::Register;

pub(crate) fn parse_utype32(opcode: &u8, rd: &u8, imm: &u32) -> Result<ParsedInstruction32, &'static str> {
    match opcode {
        0b0110111 => Ok(ParsedInstruction32::lui {
            rd: Register::try_from(*rd)?,
            imm: *imm as i32,
        }),
        0b0010111 => Ok(ParsedInstruction32::auipc {
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
    fn test_parse_utype32_lui() {
        let result = parse_utype32(&0b0110111, &0b00010, &0b00000000000000000000000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::lui {
            rd: Register::try_from(0b00010).unwrap(),
            imm: 0b00000000000000000000000000000001 as i32,
        });
    }

    #[test]
    fn test_parse_utype32_auipc() {
        let result = parse_utype32(&0b0010111, &0b00010, &0b00000000000000000000000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::auipc {
            rd: Register::try_from(0b00010).unwrap(),
            imm: 0b00000000000000000000000000000001 as i32,
        });
    }

    #[test]
    fn test_parse_utype32_invalid_opcode() {
        let result = parse_utype32(&0b0000000, &0b00010, &0b00000000000000000000000000000001);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid opcode"));
    }
}