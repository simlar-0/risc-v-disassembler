#[cfg(test)]
mod tests {
    use risc_v_disassembler::parse;
    use risc_v_disassembler::{parsed_instructions::*, ParsedInstruction32};

    /// Returns a vector of tuples containing (instruction hex, expected ParsedInstruction32) for RV32I instructions, using numbered registers
    fn get_rv32i_be_test_cases() -> Vec<(u32, ParsedInstruction32)> {
        vec![
            // R-type instructions
            (
                0x00B50533,
                ParsedInstruction32::add(add {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x40B50533,
                ParsedInstruction32::sub(sub {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x00B52533,
                ParsedInstruction32::slt(slt {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x00B53533,
                ParsedInstruction32::sltu(sltu {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x00B54533,
                ParsedInstruction32::xor(xor {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x00B56533,
                ParsedInstruction32::or(or {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x00B57533,
                ParsedInstruction32::and(and {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x00B55533,
                ParsedInstruction32::srl(srl {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x40B55533,
                ParsedInstruction32::sra(sra {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            (
                0x00B51533,
                ParsedInstruction32::sll(sll {
                    rd: "x10",
                    rs1: "x10",
                    rs2: "x11",
                }),
            ),
            // I-type instructions
            (
                0x00A50513,
                ParsedInstruction32::addi(addi {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A52513,
                ParsedInstruction32::slti(slti {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A53513,
                ParsedInstruction32::sltiu(sltiu {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A54513,
                ParsedInstruction32::xori(xori {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A56513,
                ParsedInstruction32::ori(ori {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A57513,
                ParsedInstruction32::andi(andi {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00451513,
                ParsedInstruction32::slli(slli {
                    rd: "x10",
                    rs1: "x10",
                    shamt: 4,
                }),
            ),
            (
                0x00455513,
                ParsedInstruction32::srli(srli {
                    rd: "x10",
                    rs1: "x10",
                    shamt: 4,
                }),
            ),
            (
                0x40455513,
                ParsedInstruction32::srai(srai {
                    rd: "x10",
                    rs1: "x10",
                    shamt: 4,
                }),
            ),
            // Load instructions
            (
                0x00A50503,
                ParsedInstruction32::lb(lb {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A51503,
                ParsedInstruction32::lh(lh {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A52503,
                ParsedInstruction32::lw(lw {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A54503,
                ParsedInstruction32::lbu(lbu {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A55503,
                ParsedInstruction32::lhu(lhu {
                    rd: "x10",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
            // S-type instructions
            (
                0x00A50523,
                ParsedInstruction32::sb(sb {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A51523,
                ParsedInstruction32::sh(sh {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00A52523,
                ParsedInstruction32::sw(sw {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            // B-type instructions
            (
                0x00a50563,
                ParsedInstruction32::beq(beq {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00a51563,
                ParsedInstruction32::bne(bne {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00a54563,
                ParsedInstruction32::blt(blt {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00a55563,
                ParsedInstruction32::bge(bge {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00a56563,
                ParsedInstruction32::bltu(bltu {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            (
                0x00a57563,
                ParsedInstruction32::bgeu(bgeu {
                    rs1: "x10",
                    rs2: "x10",
                    imm: 10,
                }),
            ),
            // J-type instruction
            (
                0x010000EF,
                ParsedInstruction32::jal(jal { rd: "x1", imm: 16 }),
            ),
            // U-type instructions
            (
                0x000100b7,
                ParsedInstruction32::lui(lui {
                    rd: "x1",
                    imm: 16 << 12,
                }),
            ),
            (
                0x00010097,
                ParsedInstruction32::auipc(auipc {
                    rd: "x1",
                    imm: 16 << 12,
                }),
            ),
            // I-type jump instruction
            (
                0x00A500E7,
                ParsedInstruction32::jalr(jalr {
                    rd: "x1",
                    rs1: "x10",
                    imm: 10,
                }),
            ),
        ]
    }

    #[test]
    fn test_rv32i_instructions_le() {
        for (hex, expected) in get_rv32i_be_test_cases() {
            let result = parse(&hex.to_le_bytes(), false, false);
            assert!(result.is_ok(), "Failed to parse instruction {:#010x}", hex);
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn test_rv32i_instructions_be() {
        for (hex, expected) in get_rv32i_be_test_cases() {
            let result = parse(&hex.to_be_bytes(), true, false);
            assert!(result.is_ok(), "Failed to parse instruction {:#010x}", hex);
            assert_eq!(result.unwrap(), expected);
        }
    }

    /// Returns a vector of tuples containing (instruction hex, expected ParsedInstruction32) for RV32I instructions, using ABI registers
    fn get_rv32i_be_test_cases_abi() -> Vec<(u32, ParsedInstruction32)> {
        vec![
            // R-type instructions
            (
                0x00B50533,
                ParsedInstruction32::add(add {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x40B50533,
                ParsedInstruction32::sub(sub {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x00B52533,
                ParsedInstruction32::slt(slt {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x00B53533,
                ParsedInstruction32::sltu(sltu {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x00B54533,
                ParsedInstruction32::xor(xor {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x00B56533,
                ParsedInstruction32::or(or {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x00B57533,
                ParsedInstruction32::and(and {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x00B55533,
                ParsedInstruction32::srl(srl {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x40B55533,
                ParsedInstruction32::sra(sra {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            (
                0x00B51533,
                ParsedInstruction32::sll(sll {
                    rd: "a0",
                    rs1: "a0",
                    rs2: "a1",
                }),
            ),
            // I-type instructions
            (
                0x00A50513,
                ParsedInstruction32::addi(addi {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A52513,
                ParsedInstruction32::slti(slti {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A53513,
                ParsedInstruction32::sltiu(sltiu {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A54513,
                ParsedInstruction32::xori(xori {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A56513,
                ParsedInstruction32::ori(ori {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A57513,
                ParsedInstruction32::andi(andi {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00451513,
                ParsedInstruction32::slli(slli {
                    rd: "a0",
                    rs1: "a0",
                    shamt: 4,
                }),
            ),
            (
                0x00455513,
                ParsedInstruction32::srli(srli {
                    rd: "a0",
                    rs1: "a0",
                    shamt: 4,
                }),
            ),
            (
                0x40455513,
                ParsedInstruction32::srai(srai {
                    rd: "a0",
                    rs1: "a0",
                    shamt: 4,
                }),
            ),
            // Load instructions
            (
                0x00A50503,
                ParsedInstruction32::lb(lb {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A51503,
                ParsedInstruction32::lh(lh {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A52503,
                ParsedInstruction32::lw(lw {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A54503,
                ParsedInstruction32::lbu(lbu {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A55503,
                ParsedInstruction32::lhu(lhu {
                    rd: "a0",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
            // S-type instructions
            (
                0x00A50523,
                ParsedInstruction32::sb(sb {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A51523,
                ParsedInstruction32::sh(sh {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00A52523,
                ParsedInstruction32::sw(sw {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            // B-type instructions
            (
                0x00a50563,
                ParsedInstruction32::beq(beq {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00a51563,
                ParsedInstruction32::bne(bne {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00a54563,
                ParsedInstruction32::blt(blt {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00a55563,
                ParsedInstruction32::bge(bge {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00a56563,
                ParsedInstruction32::bltu(bltu {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            (
                0x00a57563,
                ParsedInstruction32::bgeu(bgeu {
                    rs1: "a0",
                    rs2: "a0",
                    imm: 10,
                }),
            ),
            // J-type instruction
            (
                0x010000EF,
                ParsedInstruction32::jal(jal { rd: "ra", imm: 16 }),
            ),
            // U-type instructions
            (
                0x000100b7,
                ParsedInstruction32::lui(lui {
                    rd: "ra",
                    imm: 16 << 12,
                }),
            ),
            (
                0x00010097,
                ParsedInstruction32::auipc(auipc {
                    rd: "ra",
                    imm: 16 << 12,
                }),
            ),
            // I-type jump instruction
            (
                0x00A500E7,
                ParsedInstruction32::jalr(jalr {
                    rd: "ra",
                    rs1: "a0",
                    imm: 10,
                }),
            ),
        ]
    }

    #[test]
    fn test_rv32i_instructions_le_abi() {
        for (hex, expected) in get_rv32i_be_test_cases_abi() {
            let result = parse(&hex.to_le_bytes(), false, true);
            assert!(result.is_ok(), "Failed to parse instruction {:#010x}", hex);
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn test_rv32i_instructions_be_abi() {
        for (hex, expected) in get_rv32i_be_test_cases_abi() {
            let result = parse(&hex.to_be_bytes(), true, true);
            assert!(result.is_ok(), "Failed to parse instruction {:#010x}", hex);
            assert_eq!(result.unwrap(), expected);
        }
    }
}
