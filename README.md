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
        rd: String,
        rs1: String,
        rs2: String,
    },
    addi {
        rd: String,
        rs1: String,
        imm: i32,
    },
}
```

## Example Usage

```Rust
 use risc_v_disassembler::{
     parse,
     ParsedInstruction32,
     parsed_instructions::*
 };

 let bytes = [0x93, 0x00, 0x51, 0x00];
 let is_big_endian = false;
 let use_abi_register_names = false;
 let parsed_instruction = parse(&bytes, is_big_endian, use_abi_register_names).unwrap();

 assert_eq!(parsed_instruction, ParsedInstruction32::addi (addi {
     rd: "x1".to_string(),
     rs1: "x2".to_string(),
     imm: 5
 }));
```

 Or using ABI register names:

```Rust
 use risc_v_disassembler::{
     parse,
     ParsedInstruction32,
     parsed_instructions::*
 };

 let bytes = [0x93, 0x00, 0x41, 0x00];
 let is_big_endian = false;
 let use_abi_register_names = true;
 let parsed_instruction = parse(&bytes, is_big_endian, use_abi_register_names).unwrap();

 assert_eq!(parsed_instruction, ParsedInstruction32::addi (addi {
    rd: "ra".to_string(),
    rs1: "sp".to_string(),
    imm: 4
 }));
```
