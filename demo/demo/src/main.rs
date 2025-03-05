use goblin::elf::Elf;
use risc_v_disassembler::parse;
use std::{
    path::PathBuf,
    env,
    fs::File,
    io::{self, Read}
};

fn main() -> io::Result<()> {
    let root_path = env!("CARGO_MANIFEST_DIR").to_owned();
    let assets_path = PathBuf::from(root_path).join("assets");
    let elf_path = PathBuf::from(assets_path).join("demo");
    
    // Read the ELF file into a buffer
    let mut file = File::open(elf_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let raw_bytes = elf_parser(&buffer).unwrap();

    let count = raw_bytes.len();

    let mut i = 0;
    while i < count {
        let instruction: [u8;4] = raw_bytes[i..i+4].try_into().unwrap();
        //println!("{:02X} {:02X} {:02X} {:02X}", instruction[0], instruction[1], instruction[2], instruction[3]);
        let parsed_instruction = parse(&instruction).unwrap();
        println!("{}", parsed_instruction);
        i += 4;
        if count - i < 4 {
            break;
        }
    }

    Ok(())
}

fn elf_parser(buffer: &Vec<u8>) -> Result<&[u8],&str> {
    // Parse the ELF file
    match Elf::parse(buffer) {
        Ok(elf) => {
            for section in elf.section_headers.iter() {
                if let Some(Ok(name)) = elf.shdr_strtab.get(section.sh_name) {
                    if name != ".text" {continue;}
                    let offset = section.sh_offset as usize;
                    let size = section.sh_size as usize;
                    
                    if offset + size <= buffer.len() {
                        return Ok(&buffer[offset..offset + size]);
                    } else {
                        eprintln!("Invalid section boundaries for .text");
                    }
                    break;
                }
            }
            return Err("No .text section found");
        }
        Err(err) => {
            eprintln!("Failed to parse ELF file: {}", err);
            Err("Failed to parse ELF file")
        }
    }
}