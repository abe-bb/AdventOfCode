use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{AdventYear, Year};

pub fn init() -> Box<dyn AdventYear> {
    let days: Vec<Box<dyn Fn()>> = vec![Box::new(day1), Box::new(day2)];

    Box::new(Year { year: 2023, days })
}

fn day2() {
    let reader = BufReader::new(File::open("./input/2023/day2").unwrap());
    let result = day2p2_logic(reader);

    println!("result: {}", result);
}

fn _day2p1_logic(reader: impl BufRead) -> u64 {
    let mut sum = 0;

    for game in reader.lines().into_iter().map(|x| x.unwrap()) {
        sum += _day2_is_game_possible(game);
    }

    sum
}

fn day2p2_logic(reader: impl BufRead) -> u64 {
    let mut sum = 0;

    for game in reader.lines().into_iter().map(|x| x.unwrap()) {
        println!("{}", game);
        let game_power = day2_game_power(game);
        println!("{}", game_power);
        sum += game_power;
    }

    sum
}

fn _day2_is_game_possible(game: String) -> u64 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut split = game.split([' ', ':']);
    // skip the text "Game"
    split.next();
    let game_id: u64 = split.next().unwrap().parse().unwrap();
    // skip empty token
    split.next();

    while let Some(num) = split.next() {
        let num: u64 = num.parse().unwrap();
        let color = split.next().unwrap();

        match color.chars().next().unwrap() {
            'r' => {
                if num > max_red {
                    return 0;
                }
            }
            'g' => {
                if num > max_green {
                    return 0;
                }
            }
            'b' => {
                if num > max_blue {
                    return 0;
                }
            }
            _ => {
                panic!("unexpected input, expected a color name")
            }
        }
    }

    game_id
}

fn day2_game_power(game: String) -> u64 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let mut split = game.split([' ', ':']);
    // skip the text "Game"
    split.next();
    // skp the game id
    split.next().unwrap();
    // skip empty token
    split.next();

    while let Some(num) = split.next() {
        let num: u64 = num.parse().unwrap();
        let color = split.next().unwrap();

        match color.chars().next().unwrap() {
            'r' => {
                if num > max_red {
                    max_red = num;
                }
            }
            'g' => {
                if num > max_green {
                    max_green = num;
                }
            }
            'b' => {
                if num > max_blue {
                    max_blue = num;
                }
            }
            _ => {
                panic!("unexpected input, expected a color name")
            }
        }
    }

    max_red * max_green * max_blue
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
    result
}

#[cfg(test)]
mod test {
    use crate::years::year2023::{_day2p1_logic, day1_logic, day2p2_logic};

    #[test]
    fn day2p2_case1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, day2p2_logic(input.as_bytes()));
    }

    #[test]
    fn day2p1_case1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, _day2p1_logic(input.as_bytes()));
    }

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
