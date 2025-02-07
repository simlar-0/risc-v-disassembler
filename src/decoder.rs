use crate::macros::extract_bits;
use crate::instructions::{DecodedInstruction32, DecodeInstruction32, Instruction32};

impl DecodeInstruction32 for Instruction32 {
    fn decode_instruction32(&self) -> Result<DecodedInstruction32, &'static str> {
        let opcode = extract_bits!(*self, 0, 6)?;
        let decoded = match opcode {
            0b0110011 => decode_rtype32(*self)?,
            0b0010011 | 0b0000011 | 0b1100111 => decode_itype32(*self)?,
            0b0100011 => decode_stype32(*self)?,
            0b1100011 => decode_btype32(*self)?,
            0b0110111 | 0b0010111 => decode_utype32(*self)?,
            0b1101111 => decode_jtype32(*self)?,
            _ => return Err("Invalid opcode"),
        };
        Ok(decoded)
    }
}

fn decode_rtype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    let opcode = extract_bits!(instruction, 0, 6)? as u8;
    let rd = extract_bits!(instruction, 7, 11)? as u8;
    let funct3 = extract_bits!(instruction, 12, 14)? as u8;
    let rs1 = extract_bits!(instruction, 15, 19)? as u8;
    let rs2 = extract_bits!(instruction, 20, 24)? as u8;
    let funct7 = extract_bits!(instruction, 25, 31)? as u8;
    Ok(DecodedInstruction32::RType {
        opcode,
        rd,
        funct3,
        rs1,
        rs2,
        funct7,
    })
}

fn decode_itype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    let opcode = extract_bits!(instruction, 0, 6)? as u8;
    let rd = extract_bits!(instruction, 7, 11)? as u8;
    let funct3 = extract_bits!(instruction, 12, 14)? as u8;
    let rs1 = extract_bits!(instruction, 15, 19)? as u8;
    let imm= extract_bits!(instruction, 20, 31)? as u16;
    Ok(DecodedInstruction32::IType {
        opcode,
        rd,
        funct3,
        rs1,
        imm,
    })
}

fn decode_stype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    let opcode = extract_bits!(instruction, 0, 6)? as u8;
    let imm4_0 = extract_bits!(instruction, 7, 11)? as u16;
    let funct3 = extract_bits!(instruction, 12, 14)? as u8;
    let rs1 = extract_bits!(instruction, 15, 19)? as u8;
    let rs2 = extract_bits!(instruction, 20, 24)? as u8;
    let imm11_5 = extract_bits!(instruction, 25, 31)? as u16;

    let imm = imm11_5 << 5 | imm4_0; 

    Ok(DecodedInstruction32::SType {
        opcode,
        imm,
        funct3,
        rs1,
        rs2,
    })
}

fn decode_btype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    let opcode = extract_bits!(instruction, 0, 6)? as u8;
    let imm11 = extract_bits!(instruction, 7, 7)? as u16;
    let imm4_1 = extract_bits!(instruction, 8, 11)? as u16;
    let funct3 = extract_bits!(instruction, 12, 14)? as u8;
    let rs1 = extract_bits!(instruction, 15, 19)? as u8;
    let rs2 = extract_bits!(instruction, 20, 24)? as u8;
    let imm10_5 = extract_bits!(instruction, 25, 30)? as u16;
    let imm12 = extract_bits!(instruction, 31, 31)? as u16;

    let mut imm = imm12 << 12 | imm11<< 11 | imm10_5<< 5 | imm4_1 << 1;
    imm = imm >> 1;

    Ok(DecodedInstruction32::BType {
        opcode,
        imm,
        funct3,
        rs1,
        rs2,
    })
}

fn decode_utype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    let opcode = extract_bits!(instruction, 0, 6)? as u8;
    let rd = extract_bits!(instruction, 7, 11)? as u8;
    let imm = extract_bits!(instruction, 12, 31)? as u32;

    Ok(DecodedInstruction32::UType {
        opcode,
        rd,
        imm,
    })
}

fn decode_jtype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    let opcode = extract_bits!(instruction, 0, 6)? as u8;
    let rd = extract_bits!(instruction, 7, 11)? as u8;
    let imm19_12 = extract_bits!(instruction, 12, 19)? as u32;
    let imm11 = extract_bits!(instruction, 20, 20)? as u32;
    let imm10_1 = extract_bits!(instruction, 21, 30)? as u32;
    let imm20 = extract_bits!(instruction, 31, 31)? as u32;

    let imm = imm20 << 20 | imm19_12 << 12 | imm11 << 11 | imm10_1 << 1;

    Ok(DecodedInstruction32::JType {
        opcode,
        rd,
        imm,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_instruction32() {
        let rtype_instruction : Instruction32 = 0x007212b3;
        let itype_instruction : Instruction32 = 0x02a24193;
        let stype_instruction : Instruction32 = 0x045219a3;
        let btype_instruction : Instruction32 = 0x00620c63;
        let utype_instruction : Instruction32 = 0x00038197;
        let jtype_instruction : Instruction32 = 0x062001ef;

        match rtype_instruction.decode_instruction32().unwrap() {
            DecodedInstruction32::RType { .. } => {
                assert!(true);
            }
            _ => panic!("R-type instruction decoded as a different type"),
        }

        match itype_instruction.decode_instruction32().unwrap() {
            DecodedInstruction32::IType { .. } => {
                assert!(true);
            }
            _ => panic!("I-type instruction decoded as a different type"),
        }

        match stype_instruction.decode_instruction32().unwrap() {
            DecodedInstruction32::SType { .. } => {
                assert!(true);
            }
            _ => panic!("S-type instruction decoded as a different type"),
        }

        match btype_instruction.decode_instruction32().unwrap() {
            DecodedInstruction32::BType { .. } => {
                assert!(true);
            }
            _ => panic!("B-type instruction decoded as a different type"),
        }

        match utype_instruction.decode_instruction32().unwrap() {
            DecodedInstruction32::UType { .. } => {
                assert!(true);
            }
            _ => panic!("U-type instruction decoded as a different type"),
        }

        match jtype_instruction.decode_instruction32().unwrap() {
            DecodedInstruction32::JType { .. } => {
                assert!(true);
            }
            _ => panic!("J-type instruction decoded as a different type"),
        }

        
    }

    #[test]
    fn test_decode_rtype32() {
        let instruction : Instruction32 = 0x00308033;
        let result = decode_rtype32(instruction).unwrap();
        match result {
            DecodedInstruction32::RType { opcode, rd, funct3, rs1, rs2, funct7 } => {
                assert_eq!(opcode, 0b011_0011, "Opcode mismatch");
                assert_eq!(rd, 0b0_0000, "RD mismatch");
                assert_eq!(funct3, 0b000, "Funct3 mismatch");
                assert_eq!(rs1, 1, "RS1 mismatch");
                assert_eq!(rs2, 3, "RS2 mismatch");
                assert_eq!(funct7, 0b000_0000, "Funct7 mismatch");
            }
            _ => panic!("R-type instruction decoded as a different type"),
        }
    }

    #[test]
    fn test_decode_itype32() {
        let instruction : Instruction32 = 0x02a0e013;
        let result = decode_itype32(instruction).unwrap();
        match result {
            DecodedInstruction32::IType { opcode, rd, funct3, rs1, imm } => {
                assert_eq!(opcode, 0b001_0011, "Opcode mismatch");
                assert_eq!(rd, 0, "RD mismatch");
                assert_eq!(funct3, 0b110, "Funct3 mismatch");
                assert_eq!(rs1, 1, "RS1 mismatch");
                assert_eq!(imm, 42, "Immediate mismatch");
            }
            _ => panic!("I-type instruction decoded as a different type"),
        }
    }

    #[test]
    fn test_decode_stype32() {
        let instruction : Instruction32 = 0x003107a3;
        let result = decode_stype32(instruction).unwrap();
        match result {
            DecodedInstruction32::SType { opcode, imm, funct3, rs1, rs2 } => {
                assert_eq!(opcode, 0b010_0011, "Opcode mismatch");
                assert_eq!(imm, 15, "Immediate mismatch");
                assert_eq!(funct3, 0b000, "Funct3 mismatch");
                assert_eq!(rs1, 2, "RS1 mismatch");
                assert_eq!(rs2, 3, "RS2 mismatch");
            }
            _ => panic!("S-type instruction decoded as a different type"),
        }
    }

    #[test]
    fn test_decode_btype32() {
        let instruction : Instruction32 = 0x02619d63;
        let result = decode_btype32(instruction).unwrap();
        match result {
            DecodedInstruction32::BType { opcode, imm, funct3, rs1, rs2 } => {
                assert_eq!(opcode, 0b110_0011, "Opcode mismatch");
                assert_eq!(imm, 59>>1, "Immediate mismatch");
                assert_eq!(funct3, 0b001, "Funct3 mismatch");
                assert_eq!(rs1, 3, "RS1 mismatch");
                assert_eq!(rs2, 6, "RS2 mismatch");
            }
            _ => panic!("B-type instruction decoded as a different type"),
        }
    }

    #[test]
    fn test_decode_utype32() {
        let instruction : Instruction32 = 0x0005b3b7;
        let result = decode_utype32(instruction).unwrap();
        match result {
            DecodedInstruction32::UType { opcode, rd, imm } => {
                assert_eq!(opcode, 0b011_0111, "Opcode mismatch");
                assert_eq!(rd, 7, "RD mismatch");
                assert_eq!(imm, 91, "Immediate mismatch");
            }
            _ => panic!("U-type instruction decoded as a different type"),
        }
    }

    #[test]
    fn test_decode_jtype32() {
        let instruction : Instruction32 = 0x0360066f;
        let result = decode_jtype32(instruction).unwrap();
        match result {
            DecodedInstruction32::JType { opcode, rd, imm } => {
                assert_eq!(opcode, 0b110_1111, "Opcode mismatch");
                assert_eq!(rd, 12, "RD mismatch");
                assert_eq!(imm, 54, "Immediate mismatch");
            }
            _ => panic!("J-type instruction decoded as a different type"),
        }
    }
}