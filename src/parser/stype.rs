use crate::instructions::{parsed_instructions::*, ParsedInstruction32};
use crate::registers::Register;
use crate::DisassemblerError;

pub(crate) fn parse_stype32<T: Register>(
    _opcode: &u8,
    imm: &i32,
    funct3: &u8,
    rs1: &u8,
    rs2: &u8,
) -> Result<ParsedInstruction32, DisassemblerError> {
    let rs1 = T::from_u8(*rs1)?.as_str();
    let rs2 = T::from_u8(*rs2)?.as_str();

    match funct3 {
        0b000 => Ok(ParsedInstruction32::sb(sb {
            rs1,
            rs2,
            imm: *imm,
        })),
        0b001 => Ok(ParsedInstruction32::sh(sh {
            rs1,
            rs2,
            imm: *imm,
        })),
        0b010 => Ok(ParsedInstruction32::sw(sw {
            rs1,
            rs2,
            imm: *imm,
        })),
        _ => Err(DisassemblerError::InvalidFunct3(*funct3)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::NumberedRegister;

    #[test]
    fn test_parse_stype32_sb() {
        let result =
            parse_stype32::<NumberedRegister>(&0b0100011, &1, &0b000, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::sb { .. }));
    }

    #[test]
    fn test_parse_stype32_sh() {
        let result =
            parse_stype32::<NumberedRegister>(&0b0100011, &1, &0b001, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::sh { .. }));
    }

    #[test]
    fn test_parse_stype32_sw() {
        let result =
            parse_stype32::<NumberedRegister>(&0b0100011, &1, &0b010, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::sw { .. }));
    }

    #[test]
    fn test_parse_stype32_invalid_funct3() {
        let result = parse_stype32::<NumberedRegister>(&0b0100011, &1, &0b011, &0b00010, &0b00011);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidFunct3(0b011)));
    }
}
