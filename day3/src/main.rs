mod lib;

fn main() {
    let paths = lib::get_parsed_input();

    lib::part1(&paths);
    lib::part2(&paths);
}
