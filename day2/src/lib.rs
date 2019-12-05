use common::int_code_machine::IntCodeMachine;

pub fn get_parsed_input()-> IntCodeMachine {
    IntCodeMachine::new(include_str!("input/input1"))
}

// part 1 -- what is the value in register 0 when register 1 = 12 and register 2 = 2
pub fn part1(src: &IntCodeMachine) {
    println!("Part 1 = {}", with_first_registers(src, 12, 2));
}

// part 2 -- what values of r1 and r2 results in r0 == 19690720?
pub fn part2(src: &IntCodeMachine) {
    let target = 19690720;
    for i1 in 0..100usize {
        for i2 in 0..100usize {
            if with_first_registers(src, i1, i2) == target {
                println!("Part 2 = {}", 100 * i1 + i2);
                return;
            }
        }
    }
}

// parse the machine from `src`, set registers r1 and r2, run and return the
// resulting register 0 value
pub fn with_first_registers(machine: &IntCodeMachine, r1: usize, r2: usize) -> usize {
    let mut machine = machine.clone();
    machine.registers[1] = r1;
    machine.registers[2] = r2;
    machine.run();
    machine.registers[0]
}
