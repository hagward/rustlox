use crate::chunk::*;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
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
