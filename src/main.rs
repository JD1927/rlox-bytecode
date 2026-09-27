mod chunk;

use crate::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk: Chunk = Chunk::new();
    chunk.write_op_code(OpCode::Return);
    chunk.disassemble("test chunk");
    chunk.free();
}
