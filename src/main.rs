const OP_CONSTANT: u8 = 0;
const OP_ADD: u8 = 1;
const OP_SUBTRACT: u8 = 2;
const OP_MULTIPLY: u8 = 3;
const OP_DIVIDE: u8 = 4;
const OP_NEGATE: u8 = 5;
const OP_RETURN: u8 = 6;

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

enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    fn interpret(&mut self) -> InterpretResult {
        loop {
            println!("{:?}", self.stack);
            self.chunk.disassemble_instruction(self.ip);

            let instruction = self.read_byte();
            match instruction {
                OP_CONSTANT => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                OP_ADD => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                OP_SUBTRACT => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                OP_MULTIPLY => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                OP_DIVIDE => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                OP_NEGATE => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                OP_RETURN => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
                _ => println!("Unhandled instruction: {}", instruction),
            }
        }
    }

    fn binary_op(&mut self, op: u8) {}

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte();
        self.chunk.constants[index as usize]
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }
}

fn main() {
    let mut chunk = Chunk::new();

    let mut constant = chunk.add_constant(1.2);
    chunk.write_chunk(OP_CONSTANT, 123);
    chunk.write_chunk(constant as u8, 123);

    constant = chunk.add_constant(3.4);
    chunk.write_chunk(OP_CONSTANT, 123);
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OP_ADD, 123);

    constant = chunk.add_constant(5.6);
    chunk.write_chunk(OP_CONSTANT, 123);
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OP_DIVIDE, 123);
    chunk.write_chunk(OP_NEGATE, 123);

    chunk.write_chunk(OP_RETURN, 123);

    let mut vm = Vm::new(chunk);
    vm.interpret();
}
