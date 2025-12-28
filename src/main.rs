mod ebpf;
mod symbol;

use goblin::elf::Elf;
use goblin::elf::SectionHeader;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    return Err("file not specified.".into());
  }
  let elf_file_name = &args[1];
  println!("file name: {}", elf_file_name);
  let buffer = fs::read(elf_file_name)?;
  let elf = Elf::parse(&buffer)?;

  println!("entry point : 0x{:x}", elf.entry);

  let mut code = ebpf::Code::new();
  for section in &elf.section_headers {
    if section.is_executable() {
      if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
        println!("executable section found: {}", name);
        let start = section.sh_offset as usize;
        let end = start + section.sh_size as usize;
        let text_bytes = &buffer[start..end];
        let addr = section.sh_addr;
        code.load(text_bytes, addr);
      } else {
        panic!("elf.shdr_strtab.get_at() failed");
      }
    }
  }
  code.disassemble(&elf);
  Ok(())
}
