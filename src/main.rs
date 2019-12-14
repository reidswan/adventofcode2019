use clap::{App, Arg};
use day1;
use day2;
use day3;
use day4;
use day5;
use day6;
use day7;
use day8;
use std::time::Instant;

macro_rules! run_day {
    ($name:expr,$( $p:ident ),*) => {
        {
            println!("------ {} ------", $name);
            let mut i = 0;
            $(
                i += 1;
                println!("{}", i);
                time_each(vec![$p::part1, $p::part2], $p::get_parsed_input());
                println!("--------------------");
            )*
        }
    };
}

fn main() {
    let matches = App::new("MyApp")
        .arg(Arg::with_name("day").index(1).help("The day to run"))
        .get_matches();

    let day = matches.value_of("day");
    if day.is_none() {
        run_day!("All days", day1, day2, day3, day4, day5, day6, day7, day8);
    } else {
        let day = day.unwrap();
        match day {
            "1" => run_day!("Day 1", day1),
            "2" => run_day!("Day 2", day2),
            "3" => run_day!("Day 3", day3),
            "4" => run_day!("Day 4", day4),
            "5" => run_day!("Day 5", day5),
            "6" => run_day!("Day 6", day6),
            "7" => run_day!("Day 7", day7),
            "8" => run_day!("Day 8", day8),
            _ => println!("Not a valid day: {}", day),
        }
    }
}

fn time_each<T>(functions: Vec<fn(&T) -> ()>, input: T) {
    let mut times = vec![Instant::now()];
    for function in functions {
        function(&input);
        times.push(Instant::now());
    }

    let total_start = *times.first().unwrap();
    let total_end = *times.last().unwrap();

    println!("Total: {}μs", (total_end - total_start).as_micros());
    let mut prev_time = total_start;
    for (i, &current_time) in times.iter().enumerate().skip(1) {
        let duration = current_time - prev_time;
        println!("Part {}: {}μs", i, duration.as_micros());
        prev_time = current_time;
    }
}
