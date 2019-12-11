use common::int_code_machine::Machine;

pub fn get_parsed_input()-> String {
    String::from(include_str!("input/input1"))
}

pub fn part1(src: &String) {
    let mut machine = Machine::new(src, vec![1]);
    machine.run();
    println!("Part 1 = {:?}", machine.output);
}


pub fn part2(src: &String) {
    let mut machine = Machine::new(src, vec![5]);
    machine.run();
    println!("Part 2 = {:?}", machine.output);
}
