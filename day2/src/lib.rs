use common::int_code_machine::Machine;

pub fn get_parsed_input()-> String {
    String::from(include_str!("input/input1"))
}

// part 1 -- what is the value in register 0 when register 1 = 12 and register 2 = 2
pub fn part1(src: &String) {
    println!("Part 1 = {}", with_first_registers(src, 12, 2));
}

// part 2 -- what values of r1 and r2 results in r0 == 19690720?
pub fn part2(src: &String) {
    let target = 19690720;
    for i1 in 0..100i32 {
        for i2 in 0..100i32 {
            if with_first_registers(src, i1, i2) == target {
                println!("Part 2 = {}", 100 * i1 + i2);
                return;
            }
        }
    }
}

// parse the machine from `src`, set registers r1 and r2, run and return the
// resulting register 0 value
pub fn with_first_registers(machine_src: &str, r1: i32, r2: i32) -> i32 {
    let mut machine = Machine::new(machine_src, vec![]);
    machine.memory[1] = r1;
    machine.memory[2] = r2;
    machine.run();
    machine.memory[0]
}
