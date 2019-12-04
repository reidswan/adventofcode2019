fn main() {
    let input = parse_input(include_str!("input/input1"));
    part1(&input);
    part2(&input);
}

// part 1 -- what is the fuel required for just the source?
fn part1(module_masses: &Vec<u128>) {
    let fuel_required_total = module_masses
        .iter()
        .map(|&module| module / 3 - 2)
        .sum::<u128>();
    println!("Part 1 = {}", fuel_required_total);
}

fn part2(module_masses: &Vec<u128>) {
    let fuel_required_total = module_masses
        .iter()
        .map(|&module| fuel_required(module, 0))
        .sum::<u128>();
    println!("Part 2 = {}", fuel_required_total);
}

fn parse_input(input: &str) -> Vec<u128> {
    input
        .lines()
        .map(|line: &str| line.trim().parse::<u128>().unwrap())
        .collect()
}

fn fuel_required(current: u128, total: u128) -> u128 {
    if current / 3 < 2 {
        total
    } else {
        let fuel_mass = current / 3 - 2;
        fuel_required(fuel_mass, total + fuel_mass)
    }
}
