mod ebpf;

use std::fs;
use std::env;
use goblin::elf::Elf;
use ebpf::EbpfCode;

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

  for section in &elf.section_headers {
    if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
      if name == ".text" {
        let start = section.sh_offset as usize;
        let end = start + section.sh_size as usize;
        let text_bytes = &buffer[start..end];

        let code = EbpfCode::new(text_bytes);

        println!(".text");
        
      }
    }
  }
  Ok(())
}
