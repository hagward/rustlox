pub const OP_CONSTANT: u8 = 0;
pub const OP_ADD: u8 = 1;
pub const OP_SUBTRACT: u8 = 2;
pub const OP_MULTIPLY: u8 = 3;
pub const OP_DIVIDE: u8 = 4;
pub const OP_NEGATE: u8 = 5;
pub const OP_RETURN: u8 = 6;

pub type Value = f64;

pub struct Chunk {
    pub code: Vec<u8>,
    lines: Vec<u64>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn write_chunk(&mut self, byte: u8, line: u64) {
        self.code.push(byte);
        self.lines.push(line);
    }

    fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction {
            OP_CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
            OP_ADD => self.simple_instruction("OP_ADD", offset),
            OP_SUBTRACT => self.simple_instruction("OP_SUBTRACT", offset),
            OP_MULTIPLY => self.simple_instruction("OP_MULTIPLY", offset),
            OP_DIVIDE => self.simple_instruction("OP_DIVIDE", offset),
            OP_NEGATE => self.simple_instruction("OP_NEGATE", offset),
            OP_RETURN => self.simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant_index = self.code[offset + 1];
        let constant_value = self.constants[constant_index as usize];
        println!("{} {:4} '{}'", name, constant_index, constant_value);
        offset + 2
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}
