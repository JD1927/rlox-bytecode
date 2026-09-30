use crate::value::{Value, ValueArray};

#[repr(u8)]
pub enum OpCode {
    Constant,
    Return,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => OpCode::Constant,
            1 => OpCode::Return,
            _ => panic!("Invalid opcode"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        code as u8
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_op_code(&mut self, op_code: OpCode) {
        self.code.push(op_code.into());
    }

    pub fn free(&mut self) {
        self.code = Vec::new();
        self.constants.free();
    }

    pub fn disassemble<T: ToString>(&self, name: T) {
        println!("== {} ==", name.to_string());

        let mut offset: usize = 0;

        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write(value);
        self.constants.count() - 1_u8
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        let instruction: OpCode = self.code[offset].into();
        match instruction {
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::Return => self.simple_instruction("OP_RETURN", offset),
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant: u8 = self.code[offset + 1];

        print!("{name:-16} {constant:4} '");
        self.constants.print_value(constant as usize);
        println!("'");

        offset + 2
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}
