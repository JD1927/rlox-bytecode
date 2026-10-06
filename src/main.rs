mod chunk;
mod value;
mod vm;
use crate::{
    chunk::{Chunk, OpCode},
    vm::VM,
};

fn main() {
    let mut vm: VM = VM::new();
    let mut chunk: Chunk = Chunk::new();

    let constant: u8 = chunk.add_constant(1.2);
    chunk.write_op_code(OpCode::Constant, 123);
    chunk.write(constant, 123);

    chunk.write_op_code(OpCode::Return, 123);
    chunk.disassemble("test chunk");

    vm.interpret(&chunk);

    chunk.free();
    vm.free();
}
