use crate::instructions::{
    ParsedInstruction32,
    parsed_instructions::*
};
use crate::registers::Register;
use crate::DisassemblerError;

pub(crate) fn parse_jtype32(opcode: &u8, rd: &u8, imm: &i32) -> Result<ParsedInstruction32, DisassemblerError> {
    let rd = Register::try_from(*rd)?;


    match opcode {
        0b1101111 => Ok(ParsedInstruction32::jal (jal { rd, imm: *imm})),
        _ => Err(DisassemblerError::InvalidOpcode(*opcode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::ParsedInstruction32;

    #[test]
    fn test_parse_jtype32_jal() {
        let result = parse_jtype32(&0b1101111, &0b00010, &1).unwrap();
        assert!(matches!(result, ParsedInstruction32::jal {..}));
    }

    #[test]
    fn test_parse_jtype32_invalid_opcode() {
        let result = parse_jtype32(&0b0000000, &0b00010, &1);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(DisassemblerError::InvalidOpcode(0b0000000)));
    }
}