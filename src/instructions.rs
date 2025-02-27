use std::fmt;
use crate::registers::Register;
use crate::DisassemblerError;

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
        imm: i32
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
        imm: i32,
    },
    xori {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    ori {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    andi {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    slli {
        rd: Register,
        rs1: Register,
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
        imm: i32,
    },
    sltiu {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    lb {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    lh {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    lw {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    lbu {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    lhu {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    sb {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    sh {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    sw {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    beq {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    bne {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    blt {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    bge {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    bltu {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    bgeu {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    jal {
        rd: Register,
        imm: i32,
    },
    jalr {
        rd: Register,
        rs1: Register,
        imm: i32,
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

pub(crate) trait DecodeInstruction32 {
    fn decode_instruction32(&self) -> Result<DecodedInstruction32, DisassemblerError>;
}

pub(crate) trait ParseInstruction32 {
    fn parse_instruction32(&self) -> Result<ParsedInstruction32, DisassemblerError>;
}

impl fmt::Display for ParsedInstruction32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsedInstruction32::add { rd, rs1, rs2 } => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::sub { rd, rs1, rs2 } => write!(f, "sub {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::xor { rd, rs1, rs2 } => write!(f, "xor {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::or { rd, rs1, rs2 } => write!(f, "or {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::and { rd, rs1, rs2 } => write!(f, "and {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::sll { rd, rs1, rs2 } => write!(f, "sll {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::srl { rd, rs1, rs2 } => write!(f, "srl {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::sra { rd, rs1, rs2 } => write!(f, "sra {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::slt { rd, rs1, rs2 } => write!(f, "slt {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::sltu { rd, rs1, rs2 } => write!(f, "sltu {}, {}, {}", rd, rs1, rs2),
            ParsedInstruction32::addi { rd, rs1, imm } => write!(f, "addi {}, {}, {}", rd, rs1, imm),
            ParsedInstruction32::xori { rd, rs1, imm } => write!(f, "xori {}, {}, {}", rd, rs1, imm),
            ParsedInstruction32::ori { rd, rs1, imm } => write!(f, "ori {}, {}, {}", rd, rs1, imm),
            ParsedInstruction32::andi { rd, rs1, imm } => write!(f, "andi {}, {}, {}", rd, rs1, imm),
            ParsedInstruction32::slli { rd, rs1, shamt } => write!(f, "slli {}, {}, {}", rd, rs1, shamt),
            ParsedInstruction32::srli { rd, rs1, shamt } => write!(f, "srli {}, {}, {}", rd, rs1, shamt),
            ParsedInstruction32::srai { rd, rs1, shamt } => write!(f, "srai {}, {}, {}", rd, rs1, shamt),
            ParsedInstruction32::slti { rd, rs1, imm } => write!(f, "slti {}, {}, {}", rd, rs1, imm),
            ParsedInstruction32::sltiu { rd, rs1, imm } => write!(f, "sltiu {}, {}, {}", rd, rs1, imm),
            ParsedInstruction32::lb { rd, rs1, imm } => write!(f, "lb {}, {}({})", rd, imm, rs1),
            ParsedInstruction32::lh { rd, rs1, imm } => write!(f, "lh {}, {}({})", rd, imm, rs1),
            ParsedInstruction32::lw { rd, rs1, imm } => write!(f, "lw {}, {}({})", rd, imm, rs1),
            ParsedInstruction32::lbu { rd, rs1, imm } => write!(f, "lbu {}, {}({})", rd, imm, rs1),
            ParsedInstruction32::lhu { rd, rs1, imm } => write!(f, "lhu {}, {}({})", rd, imm, rs1),
            ParsedInstruction32::sb { rs1, rs2, imm } => write!(f, "sb {}, {}({})", rs2, imm, rs1),
            ParsedInstruction32::sh { rs1, rs2, imm } => write!(f, "sh {}, {}({})", rs2, imm, rs1),
            ParsedInstruction32::sw { rs1, rs2, imm } => write!(f, "sw {}, {}({})", rs2, imm, rs1),
            ParsedInstruction32::beq { rs1, rs2, imm } => write!(f, "beq {}, {}, {}", rs1, rs2, imm),
            ParsedInstruction32::bne { rs1, rs2, imm } => write!(f, "bne {}, {}, {}", rs1, rs2, imm),
            ParsedInstruction32::blt { rs1, rs2, imm } => write!(f, "blt {}, {}, {}", rs1, rs2, imm),
            ParsedInstruction32::bge { rs1, rs2, imm } => write!(f, "bge {}, {}, {}", rs1, rs2, imm),
            ParsedInstruction32::bltu { rs1, rs2, imm } => write!(f, "bltu {}, {}, {}", rs1, rs2, imm),
            ParsedInstruction32::bgeu { rs1, rs2, imm } => write!(f, "bgeu {}, {}, {}", rs1, rs2, imm),
            ParsedInstruction32::jal { rd, imm } => write!(f, "jal {}, {}", rd, imm),
            ParsedInstruction32::jalr { rd, rs1, imm } => write!(f, "jalr {}, {}({})", rd, imm, rs1),
            ParsedInstruction32::lui { rd, imm } => write!(f, "lui {}, {}", rd, imm),
            ParsedInstruction32::auipc { rd, imm } => write!(f, "auipc {}, {}", rd, imm),
            ParsedInstruction32::ecall => write!(f, "ecall"),
            ParsedInstruction32::ebreak => write!(f, "ebreak"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ParsedInstruction32;
    use crate::registers::Register;

    
    #[test]
    fn test_instruction_printing() {
        let parsed_add: ParsedInstruction32 = ParsedInstruction32::add {
            rd: Register::x1,
            rs1: Register::x2,
            rs2: Register::x3,
        };
        assert_eq!(format!("{}", parsed_add), "add x1, x2, x3");

        let parsed_addi: ParsedInstruction32 = ParsedInstruction32::addi {
            rd: Register::x1,
            rs1: Register::x31,
            imm: -5,
        };
        assert_eq!(format!("{}", parsed_addi), "addi x1, x31, -5");

        let parsed_jal: ParsedInstruction32 = ParsedInstruction32::jal {
            rd: Register::x1,
            imm: 5,
        };
        assert_eq!(format!("{}", parsed_jal), "jal x1, 5");

        let parsed_ecall: ParsedInstruction32 = ParsedInstruction32::ecall;
        assert_eq!(format!("{}", parsed_ecall), "ecall");
    }
}