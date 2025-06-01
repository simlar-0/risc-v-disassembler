pub mod parsed_instructions;

use crate::{DisassemblerError, Register};
use parsed_instructions::*;
use std::fmt;

pub type Instruction32 = u32;
pub(crate) enum DecodedInstruction32 {
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
        imm: i32,
    },
    SType {
        opcode: u8,
        imm: i32,
        funct3: u8,
        rs1: u8,
        rs2: u8,
    },
    BType {
        opcode: u8,
        imm: i32,
        funct3: u8,
        rs1: u8,
        rs2: u8,
    },
    UType {
        opcode: u8,
        rd: u8,
        imm: i32,
    },
    JType {
        opcode: u8,
        rd: u8,
        imm: i32,
    },
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ParsedInstruction32 {
    add(add),
    sub(sub),
    xor(xor),
    or(or),
    and(and),
    sll(sll),
    srl(srl),
    sra(sra),
    slt(slt),
    sltu(sltu),
    addi(addi),
    xori(xori),
    ori(ori),
    andi(andi),
    slli(slli),
    srli(srli),
    srai(srai),
    slti(slti),
    sltiu(sltiu),
    lb(lb),
    lh(lh),
    lw(lw),
    lbu(lbu),
    lhu(lhu),
    sb(sb),
    sh(sh),
    sw(sw),
    beq(beq),
    bne(bne),
    blt(blt),
    bge(bge),
    bltu(bltu),
    bgeu(bgeu),
    jal(jal),
    jalr(jalr),
    lui(lui),
    auipc(auipc),
    ecall(ecall),
    ebreak(ebreak),
}

pub(crate) trait DecodeInstruction32 {
    fn decode_instruction32(&self) -> Result<DecodedInstruction32, DisassemblerError>;
}

pub(crate) trait ParseInstruction32 {
    fn parse_instruction32<T: Register>(&self) -> Result<ParsedInstruction32, DisassemblerError>;
}

impl fmt::Display for ParsedInstruction32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsedInstruction32::add(inner) => inner.fmt(f),
            ParsedInstruction32::sub(inner) => inner.fmt(f),
            ParsedInstruction32::xor(inner) => inner.fmt(f),
            ParsedInstruction32::or(inner) => inner.fmt(f),
            ParsedInstruction32::and(inner) => inner.fmt(f),
            ParsedInstruction32::sll(inner) => inner.fmt(f),
            ParsedInstruction32::srl(inner) => inner.fmt(f),
            ParsedInstruction32::sra(inner) => inner.fmt(f),
            ParsedInstruction32::slt(inner) => inner.fmt(f),
            ParsedInstruction32::sltu(inner) => inner.fmt(f),
            ParsedInstruction32::addi(inner) => inner.fmt(f),
            ParsedInstruction32::xori(inner) => inner.fmt(f),
            ParsedInstruction32::ori(inner) => inner.fmt(f),
            ParsedInstruction32::andi(inner) => inner.fmt(f),
            ParsedInstruction32::slli(inner) => inner.fmt(f),
            ParsedInstruction32::srli(inner) => inner.fmt(f),
            ParsedInstruction32::srai(inner) => inner.fmt(f),
            ParsedInstruction32::slti(inner) => inner.fmt(f),
            ParsedInstruction32::sltiu(inner) => inner.fmt(f),
            ParsedInstruction32::lb(inner) => inner.fmt(f),
            ParsedInstruction32::lh(inner) => inner.fmt(f),
            ParsedInstruction32::lw(inner) => inner.fmt(f),
            ParsedInstruction32::lbu(inner) => inner.fmt(f),
            ParsedInstruction32::lhu(inner) => inner.fmt(f),
            ParsedInstruction32::sb(inner) => inner.fmt(f),
            ParsedInstruction32::sh(inner) => inner.fmt(f),
            ParsedInstruction32::sw(inner) => inner.fmt(f),
            ParsedInstruction32::beq(inner) => inner.fmt(f),
            ParsedInstruction32::bne(inner) => inner.fmt(f),
            ParsedInstruction32::blt(inner) => inner.fmt(f),
            ParsedInstruction32::bge(inner) => inner.fmt(f),
            ParsedInstruction32::bltu(inner) => inner.fmt(f),
            ParsedInstruction32::bgeu(inner) => inner.fmt(f),
            ParsedInstruction32::jal(inner) => inner.fmt(f),
            ParsedInstruction32::jalr(inner) => inner.fmt(f),
            ParsedInstruction32::lui(inner) => inner.fmt(f),
            ParsedInstruction32::auipc(inner) => inner.fmt(f),
            ParsedInstruction32::ecall(inner) => inner.fmt(f),
            ParsedInstruction32::ebreak(inner) => inner.fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::parsed_instructions::*;
    use crate::ParsedInstruction32;

    #[test]
    fn test_instruction_printing() {
        let parsed_add: ParsedInstruction32 = ParsedInstruction32::add(add {
            rd: "x1",
            rs1: "x2",
            rs2: "x3",
        });
        assert_eq!(format!("{}", parsed_add), "add x1, x2, x3");

        let parsed_addi: ParsedInstruction32 = ParsedInstruction32::addi(addi {
            rd: "x1",
            rs1: "x31",
            imm: -5,
        });
        assert_eq!(format!("{}", parsed_addi), "addi x1, x31, -5");

        let parsed_jal: ParsedInstruction32 = ParsedInstruction32::jal(jal { rd: "x1", imm: 5 });
        assert_eq!(format!("{}", parsed_jal), "jal x1, 5");

        let parsed_ecall: ParsedInstruction32 = ParsedInstruction32::ecall(ecall {});
        assert_eq!(format!("{}", parsed_ecall), "ecall");
    }
}
