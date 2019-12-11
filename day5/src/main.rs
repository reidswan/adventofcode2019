mod lib;

fn main() {
    let input = lib::get_parsed_input();
    lib::part1(&input);
    lib::part2(&input);
}