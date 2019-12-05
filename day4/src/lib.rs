pub fn part1(src: &(u32, u32)) {
    let (start, end) = src;
    println!("Part 1 = {}", count_with_test(*start, *end+1, test_part1));
}

pub fn part2(src: &(u32, u32)) {
    let (start, end) = src;
    println!("Part 2 = {}", count_with_test(*start, *end+1, test_part2));
}

fn count_with_test(start: u32, end: u32, test: fn(u32)-> bool)-> usize {
    (start..end).filter(|i| test(*i)).count()
}

fn digitize(source: u32)-> Vec<u8> {
    let mut num = source;
    let mut digits = vec![];
    while num > 0 {
        let digit = (num % 10) as u8;
        num /= 10;
        digits.push(digit)
    };
    digits.reverse();
    digits
}

fn group_digits(digits: &Vec<u8>)-> Vec<Vec<u8>> {
    let mut groups = vec![];
    let mut prev_digit = &digits[0];
    let mut group = vec![*prev_digit];
    
    for digit in digits.iter().skip(1) {
        if digit == prev_digit {
            group.push(*digit)
        } else {
            groups.push(group);
            group = vec![*digit];
        };
        prev_digit = digit;
    }

    groups.push(group);
    groups
}

fn non_decreasing(digits: &Vec<u8>)-> bool {
    if digits.len() == 0 {
        true
    } else {
        let mut prev_digit = &digits[0];
        for digit in digits.iter().skip(1) {
            if prev_digit > digit {
                return false
            }
            prev_digit = digit
        }
        true
    }
}

fn has_any_repeat(groups: &Vec<Vec<u8>>)-> bool {
    groups.iter().any(|grp| grp.len() >= 2)
}

fn has_strict_repeat(groups: &Vec<Vec<u8>>)-> bool {
    groups.iter().any(|grp| grp.len() == 2)
}

fn test_part1(i: u32)-> bool {
    let digits = digitize(i);
    let groups = group_digits(&digits);

    digits.len() == 6 && non_decreasing(&digits) && has_any_repeat(&groups)
}

fn test_part2(i: u32)-> bool {
    let digits = digitize(i);
    let groups = group_digits(&digits);

    digits.len() == 6 && non_decreasing(&digits) && has_strict_repeat(&groups)
}

pub fn get_parsed_input()-> (u32, u32) {
    (245318, 765747)
}