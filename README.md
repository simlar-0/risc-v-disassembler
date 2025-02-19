# RISC-V Disassembler
A RISC-V disassembler that translates machine code into a Rust enum, with the main purpose of being used in [Symex](https://github.com/simlar-0/symex) symbolic execution engine.
## Supported / Planned Instruction Sets
- [x] RV32I Base Integer Instruction Set
- [ ] RV64I Base Integer Instruction Set
- [ ] RV32E Base Integer Instruction Sets
- [ ] RV64E Base Integer Instruction Sets
- [ ] RV32C Compressed Extension
## Output Format (Example)
```Rust
pub enum ParsedInstruction32 {
    add {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    addi {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
}
```
where:
```Rust
pub enum Register {
    x0 = 0,
    x1 = 1,
    x2 = 2,
    x3 = 3,
    x4 = 4,
    .
    .
    .
    x31 = 31,
}
```
## Example Usage
```Rust
use risc_v_disassembler::{parse, ParsedInstruction32, Register};

let bytes = [0x93, 0x00, 0x51, 0x00];
let parsed_instruction = parse(&bytes).unwrap();

assert_eq!(parsed_instruction, ParsedInstruction32::addi {
    rd: Register::x1,
    rs1: Register::x2,
    imm: 5
});
```