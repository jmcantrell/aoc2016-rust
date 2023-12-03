use super::{Input, Instruction, Register, Value};

pub type Memory = [Value; 4];

#[derive(Debug, Clone, Default)]
pub struct Computer(Memory);

impl Computer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, register: Register) -> Value {
        self.0[register.index()]
    }

    pub fn set(&mut self, register: Register, value: Value) {
        self.0[register.index()] = value;
    }

    pub fn add(&mut self, register: Register, value: Value) {
        self.0[register.index()] += value;
    }

    pub fn run(&mut self, instructions: &[Instruction]) {
        let mut pointer = 0;

        while pointer < instructions.len() {
            match instructions[pointer] {
                Instruction::Copy(input, output) => match input {
                    Input::Value(value) => {
                        self.set(output, value);
                    }
                    Input::Register(register) => {
                        self.set(output, self.get(register));
                    }
                },
                Instruction::Increment(register) => {
                    self.add(register, 1);
                }
                Instruction::Decrement(register) => {
                    self.add(register, -1);
                }
                Instruction::JumpNonZero(input, offset) => {
                    let value = match input {
                        Input::Value(value) => value,
                        Input::Register(register) => self.get(register),
                    };

                    if value != 0 {
                        pointer = pointer.saturating_add_signed(offset);
                        continue;
                    }
                }
            }

            pointer += 1;
        }
    }
}
