use std::fs;
use goblin::elf::Elf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let elf_file_name = "plist";
  let buffer = fs::read(elf_file_name)?;
  let elf = Elf::parse(&buffer)?;
  
  println!("entry point : 0x{:x}", elf.entry);

  for section in &elf.section_headers {
    if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
      if name == ".text" {
        let start = section.sh_offset as usize;
        let end = start + section.sh_size as usize;
        let text_bytes = &buffer[start..end];

        println!(".text");
        for (i, byte) in text_bytes.iter().take(16).enumerate() {
          print!("{:02x} ", byte);
          if(i + 1) % 8 == 0 {
            println!();
          }
        }
        println!();
      }
    }
  }

  Ok(())
}
