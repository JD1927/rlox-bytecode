mod chunk;
mod value;
use crate::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk: Chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_op_code(OpCode::Constant, 123);
    chunk.write(constant, 123);

    chunk.write_op_code(OpCode::Return, 123);
    chunk.disassemble("test chunk");
    chunk.free();
}
