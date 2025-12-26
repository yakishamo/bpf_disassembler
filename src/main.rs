mod ebpf;
mod symbol;

use goblin::container::Ctx;
use goblin::elf::Elf;
use goblin::elf64::section_header::SHT_SYMTAB;
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
	let container = elf.header.container().unwrap();
	let endian = elf.header.endianness().unwrap();
	let ctx = Ctx::new(container,endian);

  println!("entry point : 0x{:x}", elf.entry);

	let mut code = ebpf::Code::new();
	let mut symtab_sh: Option<symbol::SymbolTable> = None;
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
    } else if section.sh_type == SHT_SYMTAB {
			println!("symbol table section found.");
			let offset = section.sh_offset as usize;
			let count = (section.sh_size / section.sh_entsize) as usize;
			symtab_sh = Some(symbol::SymbolTable::new(&buffer, offset, count, ctx));
		}
  }
	let symbol_table = symtab_sh.expect("symbol table not found.");
	println!("{}", symbol_table.get_by_addr(elf.entry).expect("symbol search failed"));
  code.disassemble(symbol_table);
  Ok(())
}
