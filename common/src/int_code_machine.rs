use crate::digits::*;

#[derive(Copy, Clone)]
enum ParameterMode {
    Positional,
    Immediate,
}

struct Parameter {
    value: i32,
    mode: ParameterMode,
}

impl Parameter {
    fn new(value: i32, mode: ParameterMode) -> Self {
        Parameter { value, mode }
    }
}

enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mult(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpTrue(Parameter, Parameter),
    JumpFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equal(Parameter, Parameter, Parameter),
    Halt,
}

impl Instruction {
    fn size(&self) -> usize {
        use Instruction::*;
        match self {
            Add { .. } | Mult { .. } | LessThan { .. } | Equal { .. } => 4,
            Input { .. } | Output { .. } => 2,
            JumpFalse { .. } | JumpTrue { .. } => 3,
            Halt => 0,
        }
    }
}

fn get_or_else<T>(src: &Vec<T>, index: usize, default: T) -> T
where
    T: Copy,
{
    if src.len() > index {
        src[index]
    } else {
        default
    }
}

fn get_parameters(
    src: &[i32],
    modes: &Vec<ParameterMode>,
    count: usize,
) -> (Option<Parameter>, Option<Parameter>, Option<Parameter>) {
    let p1 = Some(Parameter::new(
        src[0],
        get_or_else(modes, 0, ParameterMode::Positional),
    ));
    let p2 = if count > 1 {
        Some(Parameter::new(
            src[1],
            get_or_else(modes, 1, ParameterMode::Positional),
        ))
    } else {
        None
    };
    let p3 = if count > 2 {
        Some(Parameter::new(
            src[2],
            get_or_else(modes, 2, ParameterMode::Positional),
        ))
    } else {
        None
    };
    (p1, p2, p3)
}

impl Instruction {
    fn decode(mem: &[i32]) -> Instruction {
        use Instruction::*;

        let instruction_code = mem[0] % 100;
        let parameter_modes = (mem[0] / 100)
            .digits_reversed()
            .map(|d| match d {
                0 => ParameterMode::Positional,
                1 => ParameterMode::Immediate,
                _ => panic!("Not a valid parameter mode: {}", d),
            })
            .collect::<Vec<ParameterMode>>();

        match instruction_code {
            1 => {
                let (p1, p2, p3) = get_parameters(&mem[1..], &parameter_modes, 3);
                Add(p1.unwrap(), p2.unwrap(), p3.unwrap())
            }
            2 => {
                let (p1, p2, p3) = get_parameters(&mem[1..], &parameter_modes, 3);
                Mult(p1.unwrap(), p2.unwrap(), p3.unwrap())
            }
            3 => {
                let (p1, _, _) = get_parameters(&mem[1..], &parameter_modes, 1);
                Input(p1.unwrap())
            }
            4 => {
                let (p1, _, _) = get_parameters(&mem[1..], &parameter_modes, 1);
                Output(p1.unwrap())
            }
            5 => {
                let (p1, p2, _) = get_parameters(&mem[1..], &parameter_modes, 2);
                JumpTrue(p1.unwrap(), p2.unwrap())
            }
            6 => {
                let (p1, p2, _) = get_parameters(&mem[1..], &parameter_modes, 2);
                JumpFalse(p1.unwrap(), p2.unwrap())
            }
            7 => {
                let (p1, p2, p3) = get_parameters(&mem[1..], &parameter_modes, 3);
                LessThan(p1.unwrap(), p2.unwrap(), p3.unwrap())
            }
            8 => {
                let (p1, p2, p3) = get_parameters(&mem[1..], &parameter_modes, 3);
                Equal(p1.unwrap(), p2.unwrap(), p3.unwrap())
            }
            99 => Halt,
            _ => panic!("Invalid opcode: {}", instruction_code),
        }
    }
}

pub struct Machine {
    pub memory: Vec<i32>,
    mem_ptr: usize,
    input: Vec<i32>,
    input_ptr: usize,
    pub output: Vec<i32>,
}

impl Machine {
    pub fn new(src: &str, input: Vec<i32>) -> Machine {
        let memory = src
            .split(',')
            .map(|code| code.trim().parse::<i32>())
            .collect::<Result<Vec<_>, _>>();
        if let Ok(memory) = memory {
            Machine {
                memory,
                input,
                input_ptr: 0,
                mem_ptr: 0,
                output: vec![],
            }
        } else {
            panic!("Failed to parse! {:?}", memory);
        }
    }

    pub fn run(&mut self) {
        use Instruction::*;
        loop {
            let instruction = Instruction::decode(&self.memory[self.mem_ptr..]);
            let mut should_increment_ptr = true;
            match &instruction {
                Halt => return,
                Add(a, b, dest) => {
                    let sum = self.resolve(a) + self.resolve(b);
                    self.memory[dest.value as usize] = sum;
                }
                Mult(a, b, dest) => {
                    let prod = self.resolve(a) * self.resolve(b);
                    self.memory[dest.value as usize] = prod;
                }
                Input(dest) => {
                    let value = self.input[self.input_ptr];
                    self.input_ptr += 1;
                    self.memory[dest.value as usize] = value;
                }
                Output(dest) => {
                    let value = self.resolve(dest);
                    self.output.push(value);
                }
                JumpTrue(check, dest) => {
                    if self.resolve(check) != 0 {
                        should_increment_ptr = false;
                        self.mem_ptr = self.resolve(dest) as usize;
                    }
                }
                JumpFalse(check, dest) => {
                    if self.resolve(check) == 0 {
                        should_increment_ptr = false;
                        self.mem_ptr = self.resolve(dest) as usize;
                    }
                }
                LessThan(a, b, dest) => {
                    self.memory[dest.value as usize] = if self.resolve(a) < self.resolve(b) {
                        1
                    } else {
                        0
                    };
                }
                Equal(a, b, dest) => {
                    self.memory[dest.value as usize] = if self.resolve(a) == self.resolve(b) {
                        1
                    } else {
                        0
                    };
                }
            }

            if should_increment_ptr {
                self.mem_ptr += instruction.size();
            }
        }
    }

    fn resolve(&self, parameter: &Parameter) -> i32 {
        match parameter.mode {
            ParameterMode::Immediate => parameter.value,
            _ => self.memory[parameter.value as usize],
        }
    }
}
