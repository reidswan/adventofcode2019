pub const ADD: usize = 1;
pub const MUL: usize = 2;
pub const END: usize = 99;

#[derive(Clone)]
pub struct IntCodeMachine {
    pub registers: Vec<usize>,
    pointer: usize,
}

impl IntCodeMachine {
    pub fn new(source: &str) -> Self {
        IntCodeMachine {
            registers: source
                .split(',')
                .filter_map(|code| code.parse::<usize>().ok())
                .collect(),
            pointer: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.registers[self.pointer] {
                ADD => {
                    let src1 = self.registers[self.pointer + 1];
                    let src2 = self.registers[self.pointer + 2];
                    let dest = self.registers[self.pointer + 3];
                    let sum = self.registers[src1] + self.registers[src2];
                    self.registers[dest] = sum;
                }
                MUL => {
                    let src1 = self.registers[self.pointer + 1];
                    let src2 = self.registers[self.pointer + 2];
                    let dest = self.registers[self.pointer + 3];
                    let prod = self.registers[src1] * self.registers[src2];
                    self.registers[dest] = prod;
                }
                END => break,
                _ => panic!(
                    "Unexpected OP CODE {} at {}",
                    self.registers[self.pointer], self.pointer
                ),
            }
            self.pointer += 4;
        }
    }
}
