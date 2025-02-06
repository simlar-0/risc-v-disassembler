use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::helpers::extract_bits;

pub(crate) fn parse_itype32(opcode: &u8, rd: &u8, funct3: &u8, rs1: &u8, imm: &i16) -> Result<ParsedInstruction32, &'static str> {
    match opcode {
        0b0000011 => parse_itype32_load(funct3, rd, rs1, imm),
        0b0010011 => parse_itype32_alu(funct3, rd, rs1, imm),
        _ => Err("Invalid opcode"),
    }
}

fn parse_itype32_load(funct3: &u8, rd: &u8, rs1: &u8, imm: &i16) -> Result<ParsedInstruction32, &'static str> {
    match funct3 {
        0b000 => Ok(ParsedInstruction32::lb {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b001 => Ok(ParsedInstruction32::lh {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b010 => Ok(ParsedInstruction32::lw {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b100 => Ok(ParsedInstruction32::lbu {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b101 => Ok(ParsedInstruction32::lhu {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        _ => Err("Invalid funct3"),
    }
}

fn parse_itype32_alu(funct3: &u8, rd: &u8, rs1: &u8, imm: &i16) -> Result<ParsedInstruction32, &'static str> {
    let imm_upper_bits = extract_bits(*imm, 5, 11)?;
    let shamt = extract_bits(*imm, 0, 4)?;

    match funct3 {
        0b000 => Ok(ParsedInstruction32::addi {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b001 => Ok(ParsedInstruction32::slli {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            shamt: shamt as u8,
        }),
        0b010 => Ok(ParsedInstruction32::slti {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b011 => Ok(ParsedInstruction32::sltiu {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b100 => Ok(ParsedInstruction32::xori {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b101 => {
            match imm_upper_bits {
                0b0000000 => Ok(ParsedInstruction32::srli {
                    rd: Register::try_from(*rd)?,
                    rs1: Register::try_from(*rs1)?,
                    shamt: *imm as u8,
                }),
                0b0100000 => Ok(ParsedInstruction32::srai {
                    rd: Register::try_from(*rd)?,
                    rs1: Register::try_from(*rs1)?,
                    shamt: *imm as u8,
                }),
                _ => Err("Invalid imm"),
            }
        },
        0b110 => Ok(ParsedInstruction32::ori {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        0b111 => Ok(ParsedInstruction32::andi {
            rd: Register::try_from(*rd)?,
            rs1: Register::try_from(*rs1)?,
            imm: *imm,
        }),
        _ => Err("Invalid funct3"),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::Register;

    #[test]
    fn test_parse_itype32_load_lb() {
        let result = parse_itype32_load(&0b000, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::lb {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_load_lh() {
        let result = parse_itype32_load(&0b001, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::lh {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_load_lw() {
        let result = parse_itype32_load(&0b010, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::lw {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_load_lbu() {
        let result = parse_itype32_load(&0b100, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::lbu {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_load_lhu() {
        let result = parse_itype32_load(&0b101, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::lhu {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_load_invalid_funct3() {
        let result = parse_itype32_load(&0b110, &0b00001, &0b00010, &0b000000000001);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid funct3"));
    }

    #[test]
    fn test_parse_itype32_alu_addi() {
        let result = parse_itype32_alu(&0b000, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::addi {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_slli() {
        let result = parse_itype32_alu(&0b001, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::slli {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            shamt: 0b00001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_slti() {
        let result = parse_itype32_alu(&0b010, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::slti {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_sltiu() {
        let result = parse_itype32_alu(&0b011, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::sltiu {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_xori() {
        let result = parse_itype32_alu(&0b100, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::xori {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_srli() {
        let result = parse_itype32_alu(&0b101, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::srli {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            shamt: 0b00001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_srai() {
        let result = parse_itype32_alu(&0b101, &0b00001, &0b00010, &0b010000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::srai {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            shamt: 0b00001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_ori() {
        let result = parse_itype32_alu(&0b110, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::ori {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_andi() {
        let result = parse_itype32_alu(&0b111, &0b00001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::andi {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_alu_invalid_funct3() {
        let result = parse_itype32_alu(&0b1000, &0b00001, &0b00010, &0b000000000001);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid funct3"));
    }

    #[test]
    fn test_parse_itype32_alu_invalid_imm() {
        let result = parse_itype32_alu(&0b101, &0b00001, &0b00010, &0b100000000001);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid imm"));
    }
}