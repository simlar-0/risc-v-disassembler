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
            (0x00451513, ParsedInstruction32::slli { rd: Register::x10, rs1: Register::x10, shamt: 4 }),
            (0x00455513, ParsedInstruction32::srli { rd: Register::x10, rs1: Register::x10, shamt: 4 }),
            (0x40455513, ParsedInstruction32::srai { rd: Register::x10, rs1: Register::x10, shamt: 4 }),

            // Load instructions
            (0x00A50503, ParsedInstruction32::lb { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A51503, ParsedInstruction32::lh { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A52503, ParsedInstruction32::lw { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A54503, ParsedInstruction32::lbu { rd: Register::x10, rs1: Register::x10, imm: 10 }),
            (0x00A55503, ParsedInstruction32::lhu { rd: Register::x10, rs1: Register::x10, imm: 10 }),

            // S-type instructions
            (0x00A50523, ParsedInstruction32::sb { rs1: Register::x10, rs2: Register::x10, imm: 10 }),
            (0x00A51523, ParsedInstruction32::sh { rs1: Register::x10, rs2: Register::x10, imm: 10 }),
            (0x00A52523, ParsedInstruction32::sw { rs1: Register::x10, rs2: Register::x10, imm: 10 }),

            // B-type instructions
            (0x00a50563, ParsedInstruction32::beq { rs1: Register::x10, rs2: Register::x10, imm: 10 }),
            (0x00a51563, ParsedInstruction32::bne { rs1: Register::x10, rs2: Register::x10, imm: 10 }),
            (0x00a54563, ParsedInstruction32::blt { rs1: Register::x10, rs2: Register::x10, imm: 10 }),
            (0x00a55563, ParsedInstruction32::bge { rs1: Register::x10, rs2: Register::x10, imm: 10 }),
            (0x00a56563, ParsedInstruction32::bltu { rs1: Register::x10, rs2: Register::x10, imm: 10 }),
            (0x00a57563, ParsedInstruction32::bgeu { rs1: Register::x10, rs2: Register::x10, imm: 10 }),

            // J-type instruction
            (0x010000EF, ParsedInstruction32::jal { rd: Register::x1, imm: 16 }),

            // U-type instructions
            (0x000100b7, ParsedInstruction32::lui { rd: Register::x1, imm: 16<<12 }),
            (0x00010097, ParsedInstruction32::auipc { rd: Register::x1, imm: 16<<12 }),

            // I-type jump instruction
            (0x00A500E7, ParsedInstruction32::jalr { rd: Register::x1, rs1: Register::x10, imm: 10 }),
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