use crate::helpers::extract_bits_from_u32;
pub type Instruction32 = u32;
pub enum DecodedInstruction32 {
    RType {
        opcode: u8,
        rd: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        funct7: u8,
    },
    IType {
        opcode: u8,
        rd: u8,
        funct3: u8,
        rs1: u8,
        imm: u16,
    },
    SType {
        opcode: u8,
        imm: u16,
        funct3: u8,
        rs1: u8,
        rs2: u8,
    },
    BType {
        opcode: u8,
        imm: u16,
        funct3: u8,
        rs1: u8,
        rs2: u8,
    },
    UType {
        opcode: u8,
        rd: u8,
        imm: u32,
    },
    JType {
        opcode: u8,
        rd: u8,
        imm: u32
    },
}

pub struct MatchedInstruction32 {
    pub name: String,
    pub instruction: DecodedInstruction32,
}

pub trait DecodeInstruction32 {
    fn decode_instruction32(&self) -> Result<DecodedInstruction32, &'static str>;
}

pub trait MatchDecodedInstruction32 {
    fn match_instruction32(&self) -> Result<MatchedInstruction32, &'static str>;
}

impl DecodeInstruction32 for Instruction32 {
    fn decode_instruction32(&self) -> Result<DecodedInstruction32, &'static str> {
        let opcode = extract_bits_from_u32(*self, 0, 6)?;
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
    todo!()
}

fn decode_itype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    todo!()
}

fn decode_stype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    todo!()
}

fn decode_btype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    todo!()
}

fn decode_utype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    todo!()
}

fn decode_jtype32(instruction: Instruction32) -> Result<DecodedInstruction32, &'static str> {
    todo!()
}