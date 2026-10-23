pub type Value = f64;

#[derive(Clone, Debug, Default)]
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
        self.values.clear();
    }

    pub fn get(&self, value: usize) -> Value {
        self.values[value]
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn print_value(&self, constant: usize) {
        print!("{}", self.values[constant]);
    }
}
