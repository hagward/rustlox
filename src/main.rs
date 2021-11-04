const OP_CONSTANT: u8 = 0;
const OP_RETURN: u8 = 1;

type Value = f64;

struct Chunk {
    code: Vec<u8>,
    lines: Vec<u64>,
    constants: Vec<Value>,
}

impl Chunk {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn write_chunk(&mut self, byte: u8, line: u64) {
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

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction {
            OP_CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
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

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OP_CONSTANT, 123);
    chunk.write_chunk(constant as u8, 123);
    chunk.write_chunk(OP_RETURN, 123);
    chunk.disassemble_chunk("test chunk");
}
