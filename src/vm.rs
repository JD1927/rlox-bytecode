use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    value::Value,
};

pub struct VM {
    pub ip: usize,
    pub stack: Vec<Value>,
}

pub enum InterpretResult {
    Ok,
    CompileErrors,
    RuntimeErrors,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new();

        if !compiler.compile(source, &mut chunk) {
            chunk.free();
            return InterpretResult::CompileErrors;
        }

        self.ip = 0;
        let result = self.run(&chunk);

        chunk.free();
        result
    }

    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        // This macro helps avoid repeating a lot of logic and also using functions as params
        macro_rules! binary_op {
            ($self:ident, $op:tt) => {
                {
                    let b = $self.stack.pop().unwrap();
                    let a = $self.stack.pop().unwrap();
                    $self.stack.push(a $op b);
                }
            };
        }

        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                for slot in &self.stack {
                    print!("[ {slot} ]");
                }
                println!();
                chunk.disassemble_instruction(self.ip);
            }

            let instruction: OpCode = self.read_byte(chunk);

            match instruction {
                OpCode::Constant => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                }
                OpCode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    break InterpretResult::Ok;
                }
                OpCode::Negate => {
                    let last = self.stack.pop().unwrap();
                    self.stack.push(-last);
                }
                OpCode::Add => binary_op!(self, +),
                OpCode::Subtract => binary_op!(self, -),
                OpCode::Multiply => binary_op!(self, *),
                OpCode::Divide => binary_op!(self, /),
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

    pub fn free(&mut self) {
        self.reset_stack();
        self.ip = 0;
    }

    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }
}
