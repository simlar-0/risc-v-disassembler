use crate::instructions::{parsed_instructions::*, ParsedInstruction32};
use crate::registers::Register;
use crate::DisassemblerError;

pub(crate) fn parse_btype32<T: Register>(
    _opcode: &u8,
    imm: &i32,
    funct3: &u8,
    rs1: &u8,
    rs2: &u8,
) -> Result<ParsedInstruction32, DisassemblerError> {
    let rs1 = T::try_from_u8(*rs1)?.as_str();
    let rs2 = T::try_from_u8(*rs2)?.as_str();

    match funct3 {
        0b000 => Ok(ParsedInstruction32::beq(beq {
            rs1,
            rs2,
            imm: *imm,
        })),
        0b001 => Ok(ParsedInstruction32::bne(bne {
            rs1,
            rs2,
            imm: *imm,
        })),
        0b100 => Ok(ParsedInstruction32::blt(blt {
            rs1,
            rs2,
            imm: *imm,
        })),
        0b101 => Ok(ParsedInstruction32::bge(bge {
            rs1,
            rs2,
            imm: *imm,
        })),
        0b110 => Ok(ParsedInstruction32::bltu(bltu {
            rs1,
            rs2,
            imm: *imm,
        })),
        0b111 => Ok(ParsedInstruction32::bgeu(bgeu {
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
    use crate::{instructions::ParsedInstruction32, registers::NumberedRegister};

    #[test]
    fn test_parse_btype32_beq() {
        let result =
            parse_btype32::<NumberedRegister>(&0b1100011, &1, &0b000, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::beq { .. }));
    }

    #[test]
    fn test_parse_btype32_bne() {
        let result =
            parse_btype32::<NumberedRegister>(&0b1100011, &1, &0b001, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::bne { .. }));
    }

    #[test]
    fn test_parse_btype32_invalid_funct3() {
        let result = parse_btype32::<NumberedRegister>(&0b1100011, &1, &0b010, &0b00010, &0b00011);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidFunct3(0b010)));
    }

    #[test]
    fn test_parse_btype32_blt() {
        let result =
            parse_btype32::<NumberedRegister>(&0b1100011, &1, &0b100, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::blt { .. }));
    }

    #[test]
    fn test_parse_btype32_bge() {
        let result =
            parse_btype32::<NumberedRegister>(&0b1100011, &1, &0b101, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::bge { .. }));
    }

    #[test]
    fn test_parse_btype32_bltu() {
        let result =
            parse_btype32::<NumberedRegister>(&0b1100011, &1, &0b110, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::bltu { .. }));
    }

    #[test]
    fn test_parse_btype32_bgeu() {
        let result =
            parse_btype32::<NumberedRegister>(&0b1100011, &1, &0b111, &0b00010, &0b00011).unwrap();
        assert!(matches!(result, ParsedInstruction32::bgeu { .. }));
    }
}
