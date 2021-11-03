const OP_RETURN: u8 = 0;

struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{number:0>width$} ", number = offset, width = 4);

        let instruction = self.code[offset];
        match instruction {
            OP_RETURN => self.simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            },
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}

fn main() {
    let chunk = Chunk {
        code: vec![OP_RETURN],
    };
    chunk.disassemble_chunk("test chunk");
}

