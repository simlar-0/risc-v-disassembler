#[cfg(test)]
mod tests {
    use risc_v_disassembler::parse;
    use risc_v_disassembler::ParsedInstruction32;
    use risc_v_disassembler::Register;

    /// Returns a vector of tuples containing (instruction hex, expected ParsedInstruction32) for RV32I instructions
    fn get_rv32i_test_cases() -> Vec<(u32, ParsedInstruction32)> {
        vec![
            // R-type instructions
            (0x00B50533, ParsedInstruction32::add { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x40B50533, ParsedInstruction32::sub { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x00B52533, ParsedInstruction32::slt { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x00B53533, ParsedInstruction32::sltu { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x00B54533, ParsedInstruction32::xor { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x00B56533, ParsedInstruction32::or { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x00B57533, ParsedInstruction32::and { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x00B55533, ParsedInstruction32::srl { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x40B55533, ParsedInstruction32::sra { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),
            (0x00B51533, ParsedInstruction32::sll { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 }),

            // I-type instructions
            (0x00A50513, ParsedInstruction32::addi { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A52513, ParsedInstruction32::slti { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A53513, ParsedInstruction32::sltiu { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A54513, ParsedInstruction32::xori { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A56513, ParsedInstruction32::ori { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A57513, ParsedInstruction32::andi { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00051513, ParsedInstruction32::slli { rd: Register::x10, rs1: Register::x10, shamt: 0 }),
            (0x00055513, ParsedInstruction32::srli { rd: Register::x10, rs1: Register::x10, shamt: 0 }),
            (0x40055513, ParsedInstruction32::srai { rd: Register::x10, rs1: Register::x10, shamt: 0 }),

            // Load instructions
            (0x00052503, ParsedInstruction32::lw { rd: Register::x10, rs1: Register::x10, imm: 0 }),
            (0x00051503, ParsedInstruction32::lh { rd: Register::x10, rs1: Register::x10, imm: 0 }),
            (0x00050503, ParsedInstruction32::lb { rd: Register::x10, rs1: Register::x10, imm: 0 }),
            (0x00055503, ParsedInstruction32::lhu { rd: Register::x10, rs1: Register::x10, imm: 0 }),
            (0x00054503, ParsedInstruction32::lbu { rd: Register::x10, rs1: Register::x10, imm: 0 }),

            // S-type instructions
            (0x00A52023, ParsedInstruction32::sw { rs1: Register::x10, rs2: Register::x10, imm: 0 }),
            (0x00A51023, ParsedInstruction32::sh { rs1: Register::x10, rs2: Register::x10, imm: 0 }),
            (0x00A50023, ParsedInstruction32::sb { rs1: Register::x10, rs2: Register::x10, imm: 0 }),

            // B-type instructions
            (0x00B50063, ParsedInstruction32::beq { rs1: Register::x10, rs2: Register::x11, imm: 0 }),
            (0x00B51063, ParsedInstruction32::bne { rs1: Register::x10, rs2: Register::x11, imm: 0 }),
            (0x00B54063, ParsedInstruction32::blt { rs1: Register::x10, rs2: Register::x11, imm: 0 }),
            (0x00B55063, ParsedInstruction32::bge { rs1: Register::x10, rs2: Register::x11, imm: 0 }),
            (0x00B56063, ParsedInstruction32::bltu { rs1: Register::x10, rs2: Register::x11, imm: 0 }),
            (0x00B57063, ParsedInstruction32::bgeu { rs1: Register::x10, rs2: Register::x11, imm: 0 }),

            // J-type instruction
            (0x000000EF, ParsedInstruction32::jal { rd: Register::x1, imm: 0 }),

            // U-type instructions
            (0x00000537, ParsedInstruction32::lui { rd: Register::x10, imm: 0 }),
            (0x00000517, ParsedInstruction32::auipc { rd: Register::x10, imm: 0 }),

            // I-type jump instruction
            (0x000500E7, ParsedInstruction32::jalr { rd: Register::x1, rs1: Register::x10, imm: 0 }),
        ]
    }

    #[test]
    fn test_rv32i_instructions() {
        for (hex, expected) in get_rv32i_test_cases() {
            let result = parse(&hex.to_le_bytes());
            assert!(result.is_ok(), "Failed to parse instruction {:#010x}", hex);
            assert_eq!(result.unwrap(), expected);
        }
    }
}