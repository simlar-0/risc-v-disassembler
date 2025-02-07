use crate::helpers::variable_bit_structures::VarBitInt;
use crate::registers::Register;

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
        imm: VarBitInt,
    },
    SType {
        opcode: u8,
        imm: VarBitInt,
        funct3: u8,
        rs1: u8,
        rs2: u8,
    },
    BType {
        opcode: u8,
        imm: VarBitInt,
        funct3: u8,
        rs1: u8,
        rs2: u8,
    },
    UType {
        opcode: u8,
        rd: u8,
        imm: VarBitInt,
    },
    JType {
        opcode: u8,
        rd: u8,
        imm: VarBitInt
    },
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ParsedInstruction32 {
    add {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    sub {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    xor {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    or {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    and {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    sll {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    srl {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    sra {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    slt {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    sltu {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    addi {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    xori {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    ori {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    andi {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    slli {
        rd: Register,
        rs1: Register,/// 
        shamt: u8,
    },
    srli {
        rd: Register,
        rs1: Register,
        shamt: u8,
    },
    srai {
        rd: Register,
        rs1: Register,
        shamt: u8,
    },
    slti {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    sltiu {
        rd: Register,
        rs1: Register,
        imm: u16,
    },
    lb {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    lh {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    lw {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    lbu {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    lhu {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    sb {
        rs1: Register,
        rs2: Register,
        imm: i16,
    },
    sh {
        rs1: Register,
        rs2: Register,
        imm: i16,
    },
    sw {
        rs1: Register,
        rs2: Register,
        imm: i16,
    },
    beq {
        rs1: Register,
        rs2: Register,
        imm: i16,
    },
    bne {
        rs1: Register,
        rs2: Register,
        imm: i16,
    },
    blt {
        rs1: Register,
        rs2: Register,
        imm: i16,
    },
    bge {
        rs1: Register,
        rs2: Register,
        imm: i16,
    },
    bltu {
        rs1: Register,
        rs2: Register,
        imm: u16,
    },
    bgeu {
        rs1: Register,
        rs2: Register,
        imm: u16,
    },
    jal {
        rd: Register,
        imm: i32,
    },
    jalr {
        rd: Register,
        rs1: Register,
        imm: i16,
    },
    lui {
        rd: Register,
        imm: i32,
    },
    auipc {
        rd: Register,
        imm: i32,
    },
    ecall,
    ebreak,
}

pub trait DecodeInstruction32 {
    fn decode_instruction32(&self) -> Result<DecodedInstruction32, &'static str>;
}

pub trait ParseInstruction32 {
    fn parse_instruction32(&self) -> Result<ParsedInstruction32, &'static str>;
}
