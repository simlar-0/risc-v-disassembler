use crate::instructions::ParsedInstruction32;
use crate::registers::Register;

pub(crate) fn parse_btype32(_opcode: &u8, imm: &u16, funct3: &u8, rs1: &u8, rs2: &u8) -> Result<ParsedInstruction32, &'static str> {
    match funct3 {
        0b000 => Ok(ParsedInstruction32::beq {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: *imm as i16,
        }),
        0b001 => Ok(ParsedInstruction32::bne {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: *imm as i16,
        }),
        0b100 => Ok(ParsedInstruction32::blt {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: *imm as i16,
        }),
        0b101 => Ok(ParsedInstruction32::bge {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: *imm as i16,
        }),
        0b110 => Ok(ParsedInstruction32::bltu {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: *imm,
        }),
        0b111 => Ok(ParsedInstruction32::bgeu {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: *imm,
        }),
        _ => Err("Invalid funct3"),
    }   
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::ParsedInstruction32;
    use crate::registers::Register;

    #[test]
    fn test_parse_btype32_beq() {
        let result = parse_btype32(&0b1100011, &0b000000000001, &0b000, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::beq {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: 0b000000000001 as i16,
        });
    }

    #[test]
    fn test_parse_btype32_bne() {
        let result = parse_btype32(&0b1100011, &0b000000000001, &0b001, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::bne {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: 0b000000000001 as i16,
        });
    }

    #[test]
    fn test_parse_btype32_invalid_funct3() {
        let result = parse_btype32(&0b1100011, &0b000000000001, &0b010, &0b00010, &0b00011);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid funct3"));
    }

    #[test]
    fn test_parse_btype32_blt() {
        let result = parse_btype32(&0b1100011, &0b000000000001, &0b100, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::blt {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: 0b000000000001 as i16,
        });
    }

    #[test]
    fn test_parse_btype32_bge() {
        let result = parse_btype32(&0b1100011, &0b000000000001, &0b101, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::bge {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: 0b000000000001 as i16,
        });
    }

    #[test]
    fn test_parse_btype32_bltu() {
        let result = parse_btype32(&0b1100011, &0b000000000001, &0b110, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::bltu {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_btype32_bgeu() {
        let result = parse_btype32(&0b1100011, &0b000000000001, &0b111, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::bgeu {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: 0b000000000001,
        });
    }
}
