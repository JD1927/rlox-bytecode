mod chunk;
mod value;
use crate::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk: Chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_op_code(OpCode::Constant);
    chunk.write(constant);

    chunk.write_op_code(OpCode::Return);
    chunk.disassemble("test chunk");
    chunk.free();
}
