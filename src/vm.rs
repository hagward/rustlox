use crate::{chunk::*, compiler::Compiler};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm {
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        if let Ok(chunk) = Compiler::compile(source) {
            self.ip = 0;
            return self.run(&chunk);
        }

        InterpretResult::CompileError
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            println!("{:?}", self.stack);
            chunk.disassemble_instruction(self.ip);

            let opcode: OpCode = self.read_byte(chunk).into();
            match opcode {
                OpCode::Constant => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                OpCode::Subtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                OpCode::Multiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                OpCode::Divide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                OpCode::Negate => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                OpCode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
            }
        }
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let index = self.read_byte(chunk);
        chunk.constants[index as usize]
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code[self.ip];
        self.ip += 1;
        byte
    }
}
