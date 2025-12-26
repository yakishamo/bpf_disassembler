use goblin::elf::Symtab;
use goblin::container::Ctx;

pub struct SymbolTable<'a> {
	symtab: Symtab<'a>,
}

impl<'a> SymbolTable<'a> {
	pub fn new(bytes: &'a[u8], offset: usize, count: usize, ctx: Ctx) -> Self {
		let s = Symtab::parse(bytes, offset, count, ctx).expect("Symtab::parse failed");
		Self {
			symtab: s,
		}
	}

	pub fn get_by_addr(&self, addr: u64) -> Result<String, ()> {
		for sym in self.symtab.iter() {
			if sym.st_value == addr {
				return Ok(String::from("found"));
			}
		}
		Err(())
	}
}
