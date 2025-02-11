use crate::instructions::ParsedInstruction32;
use crate::registers::Register;
use crate::helpers::variable_bit_structures::VarBitInt;

pub(crate) fn parse_stype32(_opcode: &u8, imm: &VarBitInt, funct3: &u8, rs1: &u8, rs2: &u8) -> Result<ParsedInstruction32, &'static str> {
    
    match funct3 {
        0b000 => Ok(ParsedInstruction32::sb {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: i32::try_from(*imm)?,
        }),
        0b001 => Ok(ParsedInstruction32::sh {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: i32::try_from(*imm)?,
        }),
        0b010 => Ok(ParsedInstruction32::sw {
            rs1: Register::try_from(*rs1)?,
            rs2: Register::try_from(*rs2)?,
            imm: i32::try_from(*imm)?,
        }),
        _ => Err("Invalid funct3"),
    }   
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::Register;

    #[test]
    fn test_parse_stype32_sb() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_stype32(&0b0100011, &imm, &0b000, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::sb {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: i32::try_from(imm).unwrap(),
        });
    }

    #[test]
    fn test_parse_stype32_sh() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_stype32(&0b0100011, &imm, &0b001, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::sh {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: i32::try_from(imm).unwrap(),
        });
    }

    #[test]
    fn test_parse_stype32_sw() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_stype32(&0b0100011, &imm, &0b010, &0b00010, &0b00011).unwrap();
        assert_eq!(result, ParsedInstruction32::sw {
            rs1: Register::try_from(0b00010).unwrap(),
            rs2: Register::try_from(0b00011).unwrap(),
            imm: i32::try_from(imm).unwrap(),
        });
    }

    #[test]
    fn test_parse_stype32_invalid_funct3() {
        let imm = VarBitInt::new(0b000000000001, 12);
        let result = parse_stype32(&0b0100011, &imm, &0b011, &0b00010, &0b00011);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid funct3"));
    }
}