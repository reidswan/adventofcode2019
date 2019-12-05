use std::time::Instant;
use day1;
// use day2;
// use day3;


fn main() {
    
    run_day1();

}

fn run_day1() {
    println!("------ Day 1 ------");
    let input = day1::get_parsed_input();
    time_each::<Vec<u128>>(vec![day1::part1, day1::part2], input);
}

fn time_each<T>(functions: Vec<fn(&T)->()>, input: T) {
    let mut times = vec![Instant::now()];
    for function in functions {
        function(&input);
        times.push(Instant::now());
    }

    let total_start = *times.first().unwrap();
    let total_end = *times.last().unwrap();

    println!("Total: {}us", (total_end - total_start).as_micros());
    let mut prev_time = total_start;
    for (i, &current_time) in times.iter().enumerate().skip(1) {
        let duration = current_time - prev_time;
        println!("Part {}: {}us", i, duration.as_micros());
        prev_time = current_time;
    }
}

