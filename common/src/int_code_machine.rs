use crate::digits::*;
use std::cmp::max;

#[derive(Copy, Clone)]
enum ParameterMode {
    Positional,
    Immediate,
    Relative
}

struct Parameter {
    value: i128,
    mode: ParameterMode,
}

impl Parameter {
    fn new(value: i128, mode: ParameterMode) -> Self {
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
    AdjustRelativeBase(Parameter),
    Halt,
}

impl Instruction {
    fn size(&self) -> usize {
        use Instruction::*;
        match self {
            Add { .. } | Mult { .. } | LessThan { .. } | Equal { .. } => 4,
            Input { .. } | Output { .. } | AdjustRelativeBase { .. } => 2,
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
    src: &[i128],
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
    fn decode(mem: &[i128]) -> Instruction {
        use Instruction::*;

        let instruction_code = mem[0] % 100;
        let parameter_modes = (mem[0] / 100)
            .digits_reversed()
            .map(|d| match d {
                0 => ParameterMode::Positional,
                1 => ParameterMode::Immediate,
                2 => ParameterMode::Relative,
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
            9 => {
                let (p1, _, _) = get_parameters(&mem[1..], &parameter_modes, 1);
                AdjustRelativeBase(p1.unwrap())
            }
            99 => Halt,
            _ => panic!("Invalid opcode: {}", instruction_code),
        }
    }
}

#[derive(Clone)]
pub struct Machine {
    pub memory: Vec<i128>,
    mem_ptr: usize,
    pub input: Vec<i128>,
    input_ptr: usize,
    pub output: Vec<i128>,
    await_empty_input: bool,
    relative_base: isize,
}

pub enum Status {
    Waiting, Halted
}

impl Machine {
    pub fn new(src: &str, input: Vec<i128>) -> Machine {
        let memory = src
            .split(',')
            .map(|code| code.trim().parse::<i128>())
            .collect::<Result<Vec<_>, _>>();
        if let Ok(memory) = memory {
            Machine {
                memory,
                input,
                input_ptr: 0,
                mem_ptr: 0,
                output: vec![],
                await_empty_input: false,
                relative_base: 0
            }
        } else {
            panic!("Failed to parse! {:?}", memory);
        }
    }

    pub fn wait_on_input(&mut self) {
        self.await_empty_input = true;
    }

    /// ensure that the machine has memory to at least `destination`
    fn ensure_memory(&mut self, destination: usize) {
        if destination >= self.memory.len() {
            let target_size = max(2 * self.memory.len(), destination);
            self.memory.append(&mut vec![0; target_size - self.memory.len() + 1]);
        }
    }

    fn set_memory(&mut self, destination: usize, value: i128) {
        self.ensure_memory(destination);
        self.memory[destination] = value;
    }

    fn get_memory(&mut self, destination: usize)-> i128 {
        self.ensure_memory(destination);
        self.memory[destination]
    }

    pub fn run(&mut self)-> Status {
        use Instruction::*;
        loop {
            let instruction = Instruction::decode(&self.memory[self.mem_ptr..]);
            let mut should_increment_ptr = true;
            match &instruction {
                Halt => return Status::Halted,
                Add(a, b, dest) => {
                    let sum = self.resolve(a) + self.resolve(b);
                    let mem_dest = self.resolve_as_destination(dest);
                    self.set_memory(mem_dest, sum);
                }
                Mult(a, b, dest) => {
                    let prod = self.resolve(a) * self.resolve(b);
                    let mem_dest = self.resolve_as_destination(dest);
                    self.set_memory(mem_dest, prod);
                }
                Input(dest) => {
                    if self.await_empty_input && self.input_ptr == self.input.len() {
                        return Status::Waiting;
                    }
                    let value = self.input[self.input_ptr];
                    self.input_ptr += 1;
                    let mem_dest = self.resolve_as_destination(dest);
                    self.set_memory(mem_dest, value);
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
                    let write_value = if self.resolve(a) < self.resolve(b) {
                        1
                    } else {
                        0
                    };
                    let mem_dest = self.resolve_as_destination(dest);
                    self.set_memory(mem_dest, write_value);
                }
                Equal(a, b, dest) => {
                    let write_value = if self.resolve(a) == self.resolve(b) {
                        1
                    } else {
                        0
                    };
                    let mem_dest = self.resolve_as_destination(dest);
                    self.set_memory(mem_dest, write_value);
                }
                AdjustRelativeBase(a) => {
                    let adjust_val = self.resolve(a);
                    self.relative_base += adjust_val as isize
                }
            }

            if should_increment_ptr {
                self.mem_ptr += instruction.size();
            }
        }
    }

    pub fn add_input(&mut self, new_input: i128) {
        self.input.push(new_input)
    }

    fn resolve(&mut self, parameter: &Parameter) -> i128 {
        match parameter.mode {
            ParameterMode::Immediate => parameter.value,
            _ => self.get_memory(self.resolve_as_destination(parameter))
        }
    }

    fn resolve_as_destination(&self, parameter: &Parameter)-> usize {
        match parameter.mode {
            ParameterMode::Immediate => panic!("Cannot use immediate mode as a destination!"),
            ParameterMode::Positional => parameter.value as usize,
            ParameterMode::Relative => (self.relative_base as i128 + parameter.value).abs() as usize
        }
    }
}
