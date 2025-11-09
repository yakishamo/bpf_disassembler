// Instruction classes
pub const BPF_LD: u8 = 0x00;
pub const BPF_LDX: u8 = 0x01;
pub const BPF_ST: u8 = 0x02;
pub const BPF_STX: u8 = 0x03;
pub const BPF_ALU: u8 = 0x04;
pub const BPF_JMP: u8 = 0x05;
pub const BPF_JMP32: u8 = 0x06;
pub const BPF_ALU64: u8 = 0x07;

// Arithmetic instructions
// source operand
pub const BPF_K: u8 = 0x00;
pub const BPF_X: u8 = 0x08;

// operation code
pub const BPF_ADD: u8 = 0x00;
pub const BPF_SUB: u8 = 0x10;
pub const BPF_MUL: u8 = 0x20;
pub const BPF_DIV: u8 = 0x30;
pub const BPF_OR: u8 = 0x40;
pub const BPF_AND: u8 = 0x50;
pub const BPF_LSH: u8 = 0x60;
pub const BPF_RSH: u8 = 0x70;
pub const BPF_NEG: u8 = 0x80;
pub const BPF_MOD: u8 = 0x90;
pub const BPF_XOR: u8 = 0xA0;
pub const BPF_MOV: u8 = 0xB0;
pub const BPF_ARSH: u8 = 0xC0;
pub const BPF_END: u8 = 0xD0;

// Byte swap instructions
// source
pub const BPF_TO_LE: u8 = 0x00;
pub const BPF_TO_BE: u8 = 0x08;

// Jump instructions
pub const BPF_JA: u8 = 0x00;
pub const BPF_JEQ: u8 = 0x10;
pub const BPF_JGT: u8 = 0x20;
pub const BPF_JGE: u8 = 0x30;
pub const BPF_JSET: u8 = 0x40;
pub const BPF_JNE: u8 = 0x50;
pub const BPF_JSGT: u8 = 0x60;
pub const BPF_JSGE: u8 = 0x70;
pub const BPF_CALL: u8 = 0x80;
pub const BPF_EXIT: u8 = 0x90;
pub const BPF_JLT: u8 = 0xA0;
pub const BPF_JLE: u8 = 0xB0;
pub const BPF_JSLT: u8 = 0xC0;
pub const BPF_JSLE: u8 = 0xD0;

// Load and store instructions
//   size modifier
pub const BPF_W: u8 = 0x00;
pub const BPF_H: u8 = 0x80;
pub const BPF_B: u8 = 0x10;
pub const BPF_DW: u8 = 0x18;

//   mode modifier
pub const BPF_IMM: u8 = 0x00;
pub const BPF_ABS: u8 = 0x20;
pub const BPF_IND: u8 = 0x40;
pub const BPF_MEM: u8 = 0x60;
pub const BPF_ATOMIC: u8 = 0xC0;

struct Opcode {
  byte: u8,
  is_jmp: bool,
  is_arithmetic: bool,
  is_load_store: bool,
}

impl Opcode {
  pub fn new(byte: u8) -> Self {
    let mut is_jmp: bool = false;
    let mut is_arithmetic = false;
    let mut is_load_store: bool = false;

    match byte & 0x07 {
      BPF_LD | BPF_LDX | BPF_ST | BPF_STX => {
        is_load_store = true;
      }
      BPF_ALU | BPF_ALU64 => {
        is_arithmetic = true;
      }
      BPF_JMP | BPF_JMP32 => {
        is_jmp = true;
      }
      _ => {
        panic!("unknown opcode class({}).", byte & 0x07);
      }
    }

    Self {
      byte,
      is_jmp,
      is_arithmetic,
      is_load_store,
    }
  }

  pub fn print(&self) {
    if self.is_arithmetic {
      match self.byte & 0xf0 {
        BPF_ADD => {
          print!("add");
        }
        BPF_SUB => {
          print!("sub");
        }
        BPF_MUL => {
          print!("mul");
        }
        BPF_DIV => {
          print!("div");
        }
        BPF_OR => {
          print!("or");
        }
        BPF_AND => {
          print!("and");
        }
        BPF_LSH => {
          print!("lsh");
        }
        BPF_RSH => {
          print!("rsh");
        }
        BPF_NEG => {
          print!("neg");
        }
        BPF_MOD => {
          print!("mod");
        }
        BPF_XOR => {
          print!("xor");
        }
        BPF_MOV => {
          print!("mov");
        }
        BPF_ARSH => {
          print!("arsh");
        }
        BPF_END => {
          print!("end");
        }
        _ => {
          panic!("unknown arithmetic opcode(0x{:02x})", self.byte & 0xf0);
        }
      }
    } else if self.is_jmp {
      match self.byte & 0xf0 {
        BPF_JA => {
          if self.byte & 0x0f != BPF_JMP {
            panic!("BPF_JA found but class is not BPF_JMP");
          }
          print!("ja");
        }
        BPF_JEQ => {
          print!("jeq");
        }
        BPF_JGT => {
          print!("jgt");
        }
        BPF_JGE => {
          print!("jge");
        }
        BPF_JSET => {
          print!("jset");
        }
        BPF_JNE => {
          print!("jne");
        }
        BPF_JSGT => {
          print!("jsgt");
        }
        BPF_JSGE => {
          print!("jsge");
        }
        BPF_CALL => {
          print!("call");
        }
        BPF_EXIT => {
          if self.byte & 0x0f != BPF_JMP {
            panic!("BPF_JA found but class is not BPF_JMP");
          }

          print!("exit");
        }
        BPF_JLT => {
          print!("jlt");
        }
        BPF_JLE => {
          print!("jle");
        }
        BPF_JSLT => {
          print!("jslt");
        }
        BPF_JSLE => {
          print!("jsle");
        }
        _ => {
          panic!("unknown jmp opcode(0x{:02x})", self.byte & 0xf0);
        }
      }
    } else if self.is_load_store {
      match self.byte & 0x07 {
        BPF_LD => {
          print!("ld");
        }
        BPF_LDX => {
          print!("ldx");
        }
        BPF_ST => {
          print!("st");
        }
        BPF_STX => {
          print!("stx");
        }
        _ => {
          panic!(
            "unknown load or store instruction(0x{:02x})",
            self.byte & 0x07
          );
        }
      }
    }
  }
}

struct Instruction(u64);

impl Instruction {
  pub fn imm(&self) -> u32 {
    ((self.0 >> 32) & 0xffff_ffff) as u32
  }
  pub fn offset(&self) -> u16 {
    ((self.0 >> 16) & 0xffff) as u16
  }
  pub fn src(&self) -> u8 {
    ((self.0 >> 12) & 0x0f) as u8
  }
  pub fn dst(&self) -> u8 {
    ((self.0 >> 8) & 0x0f) as u8
  }
  pub fn opcode(&self) -> u8 {
    (self.0 & 0xff) as u8
  }

  pub fn print_bytes(&self) {
    let bytes = self.0.to_le_bytes();
    for b in bytes {
      print!("{:02x} ", b);
    }
  }

  pub fn print_inst(&self) {
    let opcode = Opcode::new(self.opcode());
    opcode.print();
    println!();
  }
}

pub struct Code {
  instructions: Vec<Instruction>,
}

impl Code {
  pub fn new() -> Self {
    Self {
      instructions: Vec::new(),
    }
  }

  pub fn load(&mut self, i: &[u8]) {
    for chunk in i.chunks_exact(8) {
      let n = u64::from_le_bytes(chunk.try_into().unwrap());
      self.instructions.push(Instruction(n));
    }
  }

  pub fn new_load(i: &[u8]) -> Self {
    let mut inst = Self::new();
    inst.load(i);
    inst
  }

  pub fn disassemble(&self) {
    for (i, inst) in self.instructions.iter().enumerate() {
      print!("0x{:08x}: ", i);
      inst.print_bytes();

      inst.print_inst();
    }
  }
}
