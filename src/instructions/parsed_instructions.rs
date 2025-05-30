#![allow(non_camel_case_types)]

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct add {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct sub {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct xor {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct or {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct and {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct sll {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct srl {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct sra {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct slt {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct sltu {
    pub rd: String,
    pub rs1: String,
    pub rs2: String,
}

#[derive(Debug, PartialEq)]
pub struct addi {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct xori {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct ori {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct andi {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct slli {
    pub rd: String,
    pub rs1: String,
    pub shamt: u8,
}

#[derive(Debug, PartialEq)]
pub struct srli {
    pub rd: String,
    pub rs1: String,
    pub shamt: u8,
}

#[derive(Debug, PartialEq)]
pub struct srai {
    pub rd: String,
    pub rs1: String,
    pub shamt: u8,
}

#[derive(Debug, PartialEq)]
pub struct slti {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct sltiu {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct lb {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct lh {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct lw {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct lbu {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct lhu {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct sb {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct sh {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct sw {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct beq {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct bne {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct blt {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct bge {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct bltu {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct bgeu {
    pub rs1: String,
    pub rs2: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct jal {
    pub rd: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct jalr {
    pub rd: String,
    pub rs1: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct lui {
    pub rd: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct auipc {
    pub rd: String,
    pub imm: i32,
}

#[derive(Debug, PartialEq)]
pub struct ecall {}

#[derive(Debug, PartialEq)]
pub struct ebreak {}

impl fmt::Display for add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "add {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for sub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sub {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for xor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xor {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for or {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "or {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for and {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "and {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for sll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sll {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for srl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "srl {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for sra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sra {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for slt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "slt {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for sltu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sltu {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
}

impl fmt::Display for addi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "addi {}, {}, {}", self.rd, self.rs1, self.imm)
    }
}

impl fmt::Display for xori {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xori {}, {}, {}", self.rd, self.rs1, self.imm)
    }
}

impl fmt::Display for ori {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ori {}, {}, {}", self.rd, self.rs1, self.imm)
    }
}

impl fmt::Display for andi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "andi {}, {}, {}", self.rd, self.rs1, self.imm)
    }
}

impl fmt::Display for slli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "slli {}, {}, {}", self.rd, self.rs1, self.shamt)
    }
}

impl fmt::Display for srli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "srli {}, {}, {}", self.rd, self.rs1, self.shamt)
    }
}

impl fmt::Display for srai {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "srai {}, {}, {}", self.rd, self.rs1, self.shamt)
    }
}

impl fmt::Display for slti {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "slti {}, {}, {}", self.rd, self.rs1, self.imm)
    }
}

impl fmt::Display for sltiu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sltiu {}, {}, {}", self.rd, self.rs1, self.imm)
    }
}

impl fmt::Display for lb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lb {}, {}({})", self.rd, self.imm, self.rs1)
    }
}

impl fmt::Display for lh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lh {}, {}({})", self.rd, self.imm, self.rs1)
    }
}

impl fmt::Display for lw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lw {}, {}({})", self.rd, self.imm, self.rs1)
    }
}

impl fmt::Display for lbu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lbu {}, {}({})", self.rd, self.imm, self.rs1)
    }
}

impl fmt::Display for lhu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lhu {}, {}({})", self.rd, self.imm, self.rs1)
    }
}

impl fmt::Display for sb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sb {}, {}({})", self.rs2, self.imm, self.rs1)
    }
}

impl fmt::Display for sh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sh {}, {}({})", self.rs2, self.imm, self.rs1)
    }
}

impl fmt::Display for sw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sw {}, {}({})", self.rs2, self.imm, self.rs1)
    }
}

impl fmt::Display for beq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "beq {}, {}, {}", self.rs1, self.rs2, self.imm)
    }
}

impl fmt::Display for bne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bne {}, {}, {}", self.rs1, self.rs2, self.imm)
    }
}

impl fmt::Display for blt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "blt {}, {}, {}", self.rs1, self.rs2, self.imm)
    }
}

impl fmt::Display for bge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bge {}, {}, {}", self.rs1, self.rs2, self.imm)
    }
}

impl fmt::Display for bltu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bltu {}, {}, {}", self.rs1, self.rs2, self.imm)
    }
}

impl fmt::Display for bgeu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bgeu {}, {}, {}", self.rs1, self.rs2, self.imm)
    }
}

impl fmt::Display for jal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jal {}, {}", self.rd, self.imm)
    }
}

impl fmt::Display for jalr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jalr {}, {}({})", self.rd, self.imm, self.rs1)
    }
}

impl fmt::Display for lui {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lui {}, {}", self.rd, self.imm)
    }
}

impl fmt::Display for auipc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "auipc {}, {}", self.rd, self.imm)
    }
}

impl fmt::Display for ecall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ecall")
    }
}

impl fmt::Display for ebreak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ebreak")
    }
}

