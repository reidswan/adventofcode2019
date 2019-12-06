use common::digits::*;

pub fn part1(src: &(u32, u32)) {
    let (start, end) = src;
    println!(
        "Part 1 = {}",
        (*start..*end + 1).filter(|&i| test_part1(i)).count()
    );
}

pub fn part2(src: &(u32, u32)) {
    let (start, end) = src;
    println!(
        "Part 1 = {}",
        (*start..*end + 1).filter(|&i| test_part2(i)).count()
    );
}

fn test_part1(d: u32) -> bool {
    if d < 111111 || d > 999999 {
        return false;
    }

    let last_digit = (d % 10) as u8;
    let num = d / 10;
    let result = num.digits_reversed()
        .fold(Some((last_digit, false)), |optional_result, digit| {
            let (prev_digit, has_repeat) = optional_result?;
            if digit > prev_digit {
                None
            } else if digit == prev_digit {
                Some((digit, true))
            } else {
                Some((digit, has_repeat))
            }
        });
    match result {
        None => false,
        Some((_, has_repeat)) => has_repeat
    }
}

fn test_part2(d: u32) -> bool {
    if d < 111111 || d > 999999 {
        return false;
    }

    let mut has_strict_repeat = false;
    let mut current_repeat_length = 1;
    let mut prev_digit = d % 10;
    let mut num = d / 10;
    while num > 0 {
        let current_digit = num % 10;
        if current_digit > prev_digit {
            return false;
        } else if current_digit == prev_digit {
            current_repeat_length += 1
        } else {
            if current_repeat_length == 2 {
                has_strict_repeat = true
            }
            current_repeat_length = 1;
        }
        prev_digit = current_digit;
        num /= 10
    }

    has_strict_repeat || current_repeat_length == 2
}

pub fn get_parsed_input() -> (u32, u32) {
    (245318, 765747)
}
