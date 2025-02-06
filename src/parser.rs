use crate::instructions::{DecodedInstruction32, ParseInstruction32, ParsedInstruction32};
use crate::registers::Register;
use crate::helpers::extract_bits;

impl ParseInstruction32 for DecodedInstruction32 {
    fn parse_instruction32(&self) -> Result<ParsedInstruction32, &'static str> {
        match self {
            DecodedInstruction32::RType { opcode, rd, funct3, rs1, rs2, funct7 } => 
                parse_rtype32(opcode, rd, funct3, rs1, rs2, funct7),
            DecodedInstruction32::IType { opcode, rd, funct3, rs1, imm } =>
                parse_itype32(opcode, rd, funct3, rs1, imm),
            DecodedInstruction32::SType { opcode, imm, funct3, rs1, rs2 } =>
                parse_stype32(opcode, imm, funct3, rs1, rs2),
            DecodedInstruction32::BType { opcode, imm, funct3, rs1, rs2 } =>
                parse_btype32(opcode, imm, funct3, rs1, rs2),
            DecodedInstruction32::UType { opcode, rd, imm } =>
                parse_utype32(opcode, rd, imm),
            DecodedInstruction32::JType { opcode, rd, imm } =>
                parse_jtype32(opcode, rd, imm),
        }
    }
}

fn parse_rtype32(_opcode: &u8, rd: &u8, funct3: &u8, rs1: &u8, rs2: &u8, funct7: &u8) -> Result<ParsedInstruction32, &'static str> {
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

fn parse_itype32(_opcode: &u8, rd: &u8, funct3: &u8, rs1: &u8, imm: &i16) -> Result<ParsedInstruction32, &'static str> {
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

fn parse_stype32(opcode: &u8, imm: &i16, funct3: &u8, rs1: &u8, rs2: &u8) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

fn parse_btype32(opcode: &u8, imm: &i16, funct3: &u8, rs1: &u8, rs2: &u8) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

fn parse_utype32(opcode: &u8, rd: &u8, imm: &u32) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

fn parse_jtype32(opcode: &u8, rd: &u8, imm: &i32) -> Result<ParsedInstruction32, &'static str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::Register;

    #[test]
    fn test_parse_instruction32_rtype_add() {
        let decoded = DecodedInstruction32::RType {
            opcode: 0b0110011,
            rd: 0b00001,
            funct3: 0b000,
            rs1: 0b00010,
            rs2: 0b00011,
            funct7: 0b0000000,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert_eq!(result, ParsedInstruction32::add {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
        });
    }

    #[test]
    fn test_parse_instruction32_itype_addi() {
        let decoded = DecodedInstruction32::IType {
            opcode: 0b0010011,
            rd: 0b00001,
            funct3: 0b000,
            rs1: 0b00010,
            imm: 0b000000000001,
        };
        let result = decoded.parse_instruction32().unwrap();
        assert_eq!(result, ParsedInstruction32::addi {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

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
    #[test]
    fn test_parse_itype32_addi() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b000, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::addi {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_slli() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b001, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::slli {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            shamt: 0b00001,
        });
    }

    #[test]
    fn test_parse_itype32_slti() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b010, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::slti {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_sltiu() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b011, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::sltiu {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_xori() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b100, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::xori {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_srli() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b101, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::srli {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            shamt: 0b00001,
        });
    }

    #[test]
    fn test_parse_itype32_srai() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b101, &0b00010, &0b010000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::srai {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            shamt: 0b00001,
        });
    }

    #[test]
    fn test_parse_itype32_ori() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b110, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::ori {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_andi() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b111, &0b00010, &0b000000000001).unwrap();
        assert_eq!(result, ParsedInstruction32::andi {
            rd: Register::try_from(0b00001).unwrap(),
            rs1: Register::try_from(0b00010).unwrap(),
            imm: 0b000000000001,
        });
    }

    #[test]
    fn test_parse_itype32_invalid_funct3() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b1000, &0b00010, &0b000000000001);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid funct3"));
    }

    #[test]
    fn test_parse_itype32_invalid_imm() {
        let result = parse_itype32(&0b0010011, &0b00001, &0b101, &0b00010, &0b100000000001);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid imm"));
    }
}