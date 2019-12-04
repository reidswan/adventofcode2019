use common::int_code_machine::IntCodeMachine;

fn main() {
    let src = include_str!("input/input1");
    part1(src);
    part2(src);
}

// part 1 -- what is the value in register 0 when register 1 = 12 and register 2 = 2
fn part1(src: &str) {
    println!("Part 1 = {}", with_first_registers(src, 12, 2));
}

// part 2 -- what values of r1 and r2 results in r0 == 19690720?
fn part2(src: &str) {
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
fn with_first_registers(src: &str, r1: usize, r2: usize) -> usize {
    let mut machine = IntCodeMachine::new(src);
    machine.registers[1] = r1;
    machine.registers[2] = r2;
    machine.run();
    machine.registers[0]
}
