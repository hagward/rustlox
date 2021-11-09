mod chunk;
mod vm;

use chunk::*;
use vm::*;

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
