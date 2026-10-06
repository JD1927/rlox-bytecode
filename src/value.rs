pub type Value = f64;

#[derive(Clone, Debug)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn free(&mut self) {
        self.values = Vec::new();
    }

    pub fn get(&self, value: usize) -> Value {
        self.values[value]
    }

    pub fn count(&self) -> u8 {
        self.values.len() as u8
    }

    pub fn print_value(&self, constant: usize) {
        print!("{}", self.values[constant]);
    }
}
