use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub struct VM {
    pub ip: usize,
}

pub enum InterpretResult {
    Ok,
    // CompileErrors,
    // RuntimeErrors,
}

impl VM {
    pub fn new() -> Self {
        Self { ip: 0 }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.run(chunk)
    }

    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            chunk.disassemble_instruction(self.ip);
            let instruction: OpCode = self.read_byte(chunk);

            match instruction {
                OpCode::Constant => {
                    let constant = self.read_constant(chunk);
                    println!("{}", constant);
                }
                OpCode::Return => break InterpretResult::Ok,
            }
        }
    }

    pub fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let value: OpCode = chunk.get_code(self.ip).into();
        self.ip += 1;
        value
    }

    pub fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let index = chunk.get_code(self.ip) as usize;
        self.ip += 1;
        chunk.get_constant(index)
    }

    pub fn free(&mut self) {}
}
