use crate::instructions::ParsedInstruction32;
use crate::registers::Register;

pub(crate) fn parse_rtype32(_opcode: &u8, rd: &u8, funct3: &u8, rs1: &u8, rs2: &u8, funct7: &u8) -> Result<ParsedInstruction32, &'static str> {
    match funct3 {
        0b000 => {
            match funct7 {
                0b0000000 => Ok(ParsedInstruction32::add {
                    rd: Register::try_from(*rd)?,
                    rs1: Register::try_from(*rs1)?,
                    rs2: Register::try_from(*rs2)?,
                }),
                0b0100000 => Ok(ParsedInstruction32::sub {
                    rd: Register::try_from(*rd)?,
                    rs1: Register::try_from(*rs1)?,
                    rs2: Register::try_from(*rs2)?,
                }),
                _ => Err("Invalid funct7"),
            }
        },
        0b001 => Ok(ParsedInstruction32::sll {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
        }),
        0b010 => Ok(ParsedInstruction32::slt {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
        }),
        0b011 => Ok(ParsedInstruction32::sltu {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
        }),
        0b100 => Ok(ParsedInstruction32::xor {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
        }),
        0b101 => {
            match funct7 {
                0b0000000 => Ok(ParsedInstruction32::srl {
                    rd: Register::try_from(*rd)?,
                    rs1: Register::try_from(*rs1)?,
                    rs2: Register::try_from(*rs2)?,
                }),
                0b0100000 => Ok(ParsedInstruction32::sra {
                    rd: Register::try_from(*rd)?,
                    rs1: Register::try_from(*rs1)?,
                    rs2: Register::try_from(*rs2)?,
                }),
                _ => Err("Invalid funct7"),
            }
        },
        0b110 => Ok(ParsedInstruction32::or {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
        }),
        0b111 => Ok(ParsedInstruction32::and {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
        }),
        _ => Err("Invalid funct3"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::Register;

    #[test]
    fn test_parse_rtype32_add() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b000, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert_eq!(result, ParsedInstruction32::add {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_sub() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b000, &0b00000010, &0b00000011, &0b0100000).unwrap();
        assert_eq!(result, ParsedInstruction32::sub {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_sll() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b001, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert_eq!(result, ParsedInstruction32::sll {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_slt() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b010, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert_eq!(result, ParsedInstruction32::slt {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_sltu() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b011, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert_eq!(result, ParsedInstruction32::sltu {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_xor() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b100, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert_eq!(result, ParsedInstruction32::xor {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_srl() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b101, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert_eq!(result, ParsedInstruction32::srl {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_sra() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b101, &0b00000010, &0b00000011, &0b0100000).unwrap();
        assert_eq!(result, ParsedInstruction32::sra {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_or() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b110, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert_eq!(result, ParsedInstruction32::or {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_and() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b111, &0b00000010, &0b00000011, &0b0000000).unwrap();
        assert_eq!(result, ParsedInstruction32::and {
            rd: Register::try_from(0b00000001).unwrap(),
            rs1: Register::try_from(0b00000010).unwrap(),
            rs2: Register::try_from(0b00000011).unwrap(),
        });
    }

    #[test]
    fn test_parse_rtype32_invalid_funct3() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b1000, &0b00000010, &0b00000011, &0b0000000);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid funct3"));
    }

    #[test]
    fn test_parse_rtype32_invalid_funct7() {
        let result = parse_rtype32(&0b00000000, &0b00000001, &0b000, &0b00000010, &0b00000011, &0b1000000);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid funct7"));
    }
}