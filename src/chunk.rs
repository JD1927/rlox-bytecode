#[repr(u8)]
pub enum OpCode {
    Return,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => OpCode::Return,
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
}

impl Chunk {
    pub fn new() -> Chunk {
        Self { code: Vec::new() }
    }

    pub fn _write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_op_code(&mut self, op_code: OpCode) {
        self.code.push(op_code.into());
    }

    pub fn free(&mut self) {
        self.code = Vec::new();
    }

    pub fn disassemble<T: ToString>(&self, name: T) {
        println!("== {} ==", name.to_string());

        let mut offset = 0;

        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        println!("{offset:04} ");

        let instruction = self.code[offset].into();

        match instruction {
            OpCode::Return => self.simple_instruction("OP_RETURN", offset),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}
