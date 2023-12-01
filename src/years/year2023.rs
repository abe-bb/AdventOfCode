use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{AdventYear, Year};

pub fn init() -> Box<dyn AdventYear> {
    let days: Vec<Box<dyn Fn()>> = vec![Box::new(day1)];

    Box::new(Year { year: 2023, days })
}

fn day1() {
    let reader = BufReader::new(File::open("./input/2023/day1").unwrap());
    let result = day1_logic(reader);
    println!("{}", result);
}

// The current solution has lots of cloneing and could be seriously optimized
// by using a state machine and finding everything using a single iteration through
// each line and a state machine to keep track of the text based characters
// but this is just for AdventOfCode, so I'm not gonne put in the work to optimize
// it
fn day1_logic(reader: impl BufRead) -> u64 {
    reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| insert_digits_from_text(x))
        .map(|x| recover_calibration_value(x))
        .sum()
}

fn insert_digits_from_text(input: String) -> String {
    let digits = [
        ('0', "zero"),
        ('1', "one"),
        ('2', "two"),
        ('3', "three"),
        ('4', "four"),
        ('5', "five"),
        ('6', "six"),
        ('7', "seven"),
        ('8', "eight"),
        ('9', "nine"),
    ];

    let mut working = input;
    for digit in digits {
        let clone = working.clone();
        let spelled_digits: Vec<_> = clone.match_indices(digit.1).collect();
        for (extra, (i, _)) in spelled_digits.iter().enumerate() {
            working.insert(i + 1 + extra, digit.0);
        }
    }
    working
}

fn recover_calibration_value(line: String) -> u64 {
    let mut digits = [None, None];

    println!("{}", line);

    for c in line.chars() {
        // ignore all non digit characters
        if !c.is_ascii_digit() {
            continue;
        }

        if let None = digits[0] {
            digits[0] = Some(c);
            digits[1] = Some(c);
        } else {
            digits[1] = Some(c);
        }
    }

    let result: String = digits.into_iter().map(|x| x.unwrap()).collect();
    let result = result.parse::<u64>().unwrap();
    println!("{}", result);

    result
}

#[cfg(test)]
mod test {
    use crate::years::year2023::day1_logic;

    #[test]
    fn day1p2_case1() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, day1_logic(input.as_bytes()));
    }
}
