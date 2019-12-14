use common::int_code_machine::Machine;

pub fn get_parsed_input() -> Machine {
    Machine::new(include_str!("input/input"), vec![])
}

pub fn part1(input: &Machine) {
    let mut machine = input.clone();
    machine.add_input(1);
    machine.run();
    println!("Part 1 = {:?}", machine.output.last().unwrap());
}

pub fn part2(input: &Machine) {
    let mut machine = input.clone();
    machine.add_input(2);
    machine.run();
    println!("Part 2 = {:?}", machine.output.last().unwrap());
}
