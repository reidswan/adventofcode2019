use clap::{App, Arg};
use day1;
use day2;
use day3;
use day4;
use std::time::Instant;

fn main() {
    let matches = App::new("MyApp")
        .arg(Arg::with_name("day").index(1).help("The day to run"))
        .get_matches();

    let day = matches.value_of("day");
    if day.is_none() {
        run_day1();
        run_day2();
        run_day3();
        run_day4();
    } else {
        let day = day.unwrap();
        match day {
            "1" => run_day1(),
            "2" => run_day2(),
            "3" => run_day3(),
            "4" => run_day4(),
            _ => println!("Not a valid day: {}", day),
        }
    }
}

fn run_day1() {
    println!("------ Day 1 ------");
    let input = day1::get_parsed_input();
    time_each::<Vec<u128>>(vec![day1::part1, day1::part2], input);
}

fn run_day2() {
    println!("------ Day 2 ------");
    let input = day2::get_parsed_input();
    time_each(vec![day2::part1, day2::part2], input);
}

fn run_day3() {
    println!("------ Day 3 ------");
    let input = day3::get_parsed_input();
    time_each(vec![day3::part1, day3::part2], input)
}

fn run_day4() {
    println!("------ Day 4 ------");
    let input = day4::get_parsed_input();
    time_each(vec![day4::part1, day4::part2], input);
}

fn time_each<T>(functions: Vec<fn(&T) -> ()>, input: T) {
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
