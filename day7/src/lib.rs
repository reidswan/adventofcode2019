use common::int_code_machine::{Machine, Status};
use common::permutations::*;
use std::mem;

pub fn get_parsed_input()-> Machine {
    let input = include_str!("input/input");
    Machine::new(input, vec![])
}

pub fn part1(start_machine: &Machine) {
    let s = (0..5).collect::<Vec<_>>();
    let result = s
        .permutations()
        .fold(None, |acc, perm| {
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

pub fn part2(start_machine: &Machine) {
    
    let s = (5..10).collect::<Vec<_>>();

    let result = s
        .permutations()
        .fold(None, |acc, perm| {
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

fn init_machines(src_machine: &Machine, settings: &Vec<i128>) -> Vec<Machine> {
    let mut machines = Vec::with_capacity(settings.len());
    let mut prev_output = 0;

    for setting in settings {
        let mut machine = src_machine.clone();
        machine.wait_on_input();
        machine.input = vec![*setting, prev_output];
        machine.run();
        prev_output = machine.output[0];
        machines.push(machine)
    }

    machines
}
