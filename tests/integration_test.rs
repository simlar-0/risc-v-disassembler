#[cfg(test)]
mod tests {
    use risc_v_disassembler::parse;
    use risc_v_disassembler::{
        ParsedInstruction32,
        parsed_instructions::*
    };
    use risc_v_disassembler::Register;

    /// Returns a vector of tuples containing (instruction hex, expected ParsedInstruction32) for RV32I instructions
    fn get_rv32i_be_test_cases() -> Vec<(u32, ParsedInstruction32)> {
        vec![
            // R-type instructions
            (0x00B50533, ParsedInstruction32::add (add { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x40B50533, ParsedInstruction32::sub (sub { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x00B52533, ParsedInstruction32::slt (slt { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x00B53533, ParsedInstruction32::sltu (sltu { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x00B54533, ParsedInstruction32::xor (xor { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x00B56533, ParsedInstruction32::or (or { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x00B57533, ParsedInstruction32::and (and { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x00B55533, ParsedInstruction32::srl (srl { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x40B55533, ParsedInstruction32::sra (sra { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),
            (0x00B51533, ParsedInstruction32::sll (sll { rd: Register::x10, rs1: Register::x10, rs2: Register::x11 })),

            // I-type instructions
            (0x00A50513, ParsedInstruction32::addi (addi { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A52513, ParsedInstruction32::slti (slti { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A53513, ParsedInstruction32::sltiu (sltiu { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A54513, ParsedInstruction32::xori (xori { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A56513, ParsedInstruction32::ori (ori { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A57513, ParsedInstruction32::andi (andi { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00451513, ParsedInstruction32::slli (slli { rd: Register::x10, rs1: Register::x10, shamt: 4 })),
            (0x00455513, ParsedInstruction32::srli (srli { rd: Register::x10, rs1: Register::x10, shamt: 4 })),
            (0x40455513, ParsedInstruction32::srai (srai { rd: Register::x10, rs1: Register::x10, shamt: 4 })),

            // Load instructions
            (0x00A50503, ParsedInstruction32::lb (lb { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A51503, ParsedInstruction32::lh (lh { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A52503, ParsedInstruction32::lw (lw { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A54503, ParsedInstruction32::lbu (lbu { rd: Register::x10, rs1: Register::x10, imm: 10 })),
            (0x00A55503, ParsedInstruction32::lhu (lhu { rd: Register::x10, rs1: Register::x10, imm: 10 })),

            // S-type instructions
            (0x00A50523, ParsedInstruction32::sb (sb { rs1: Register::x10, rs2: Register::x10, imm: 10 })),
            (0x00A51523, ParsedInstruction32::sh (sh { rs1: Register::x10, rs2: Register::x10, imm: 10 })),
            (0x00A52523, ParsedInstruction32::sw (sw { rs1: Register::x10, rs2: Register::x10, imm: 10 })),

            // B-type instructions
            (0x00a50563, ParsedInstruction32::beq (beq { rs1: Register::x10, rs2: Register::x10, imm: 10 })),
            (0x00a51563, ParsedInstruction32::bne (bne { rs1: Register::x10, rs2: Register::x10, imm: 10 })),
            (0x00a54563, ParsedInstruction32::blt (blt { rs1: Register::x10, rs2: Register::x10, imm: 10 })),
            (0x00a55563, ParsedInstruction32::bge (bge { rs1: Register::x10, rs2: Register::x10, imm: 10 })),
            (0x00a56563, ParsedInstruction32::bltu (bltu { rs1: Register::x10, rs2: Register::x10, imm: 10 })),
            (0x00a57563, ParsedInstruction32::bgeu (bgeu { rs1: Register::x10, rs2: Register::x10, imm: 10 })),

            // J-type instruction
            (0x010000EF, ParsedInstruction32::jal (jal { rd: Register::x1, imm: 16 })),

            // U-type instructions
            (0x000100b7, ParsedInstruction32::lui (lui { rd: Register::x1, imm: 16<<12 })),
            (0x00010097, ParsedInstruction32::auipc (auipc { rd: Register::x1, imm: 16<<12 })),

            // I-type jump instruction
            (0x00A500E7, ParsedInstruction32::jalr (jalr { rd: Register::x1, rs1: Register::x10, imm: 10 })),
        ]
    }

    #[test]
    fn test_rv32i_instructions_le() {
        for (hex, expected) in get_rv32i_be_test_cases() {
            let result = parse(&hex.to_le_bytes(), false);
            assert!(result.is_ok(), "Failed to parse instruction {:#010x}", hex);
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn test_rv32i_instructions_be() {
        for (hex, expected) in get_rv32i_be_test_cases() {
            let result = parse(&hex.to_be_bytes(), true);
            assert!(result.is_ok(), "Failed to parse instruction {:#010x}", hex);
            assert_eq!(result.unwrap(), expected);
        }
    }

}