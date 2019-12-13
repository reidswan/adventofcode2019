use common::int_code_machine::{Machine, Status};
use common::permutations::*;
use std::mem;

pub fn part1() {
    let input = include_str!("input/input");

    let s = (0i32..5i32).collect::<Vec<_>>();
    let start_machine = Machine::new(input, vec![]);
    let result = s
        .permutations()
        .fold(None, |acc: Option<i32>, perm| {
            let mut prev_output = 0;
            for &setting in perm {
                let mut machine = start_machine.clone();
                machine.input = vec![setting, prev_output];
                machine.run();
                prev_output = machine.output[0];
            }
            if let Some(max) = acc {
                Some(if max > prev_output { max } else { prev_output })
            } else {
                Some(prev_output)
            }
        })
        .unwrap();

    println!("Part 1 = {}", result)
}

pub fn part2() {
    let input = include_str!("input/input");

    let s = (5i32..10i32).collect::<Vec<_>>();
    let mut start_machine = Machine::new(input, vec![]);
    start_machine.wait_on_input();

    let result = s
        .permutations()
        .fold(None, |acc: Option<i32>, perm| {
            let init = init_machines(&start_machine, perm);
            let mut machines = init.into_iter().map(|m| Some(m)).collect::<Vec<_>>();
            let mut current = 0;
            let mut prev_output = *machines[machines.len() - 1]
                .as_ref()
                .and_then(|a| a.output.last())
                .unwrap();
            loop {
                let mut machine = match mem::replace(&mut machines[current], None) {
                    None => break,
                    Some(m) => m,
                };
                machine.add_input(prev_output);
                let run_status = machine.run();
                prev_output = *machine.output.last().unwrap();
                if let Status::Waiting = run_status {
                    machines[current] = Some(machine);
                }
                current = (current + 1) % machines.len();
            }
            match acc {
                Some(max) => Some(if max > prev_output { max } else { prev_output }),
                _ => Some(prev_output),
            }
        })
        .unwrap();

    println!("Part 2 = {}", result)
}

fn init_machines(src_machine: &Machine, settings: &Vec<i32>) -> Vec<Machine> {
    let mut machines = Vec::with_capacity(settings.len());
    let mut prev_output = 0;

    for setting in settings {
        let mut machine = src_machine.clone();
        machine.input = vec![*setting, prev_output];
        machine.run();
        prev_output = machine.output[0];
        machines.push(machine)
    }

    machines
}
