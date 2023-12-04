use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use crate::{AdventYear, Year};

pub fn init() -> Box<dyn AdventYear> {
    let days: Vec<Box<dyn Fn()>> = vec![
        Box::new(day1),
        Box::new(day2),
        Box::new(day3),
        Box::new(day4),
    ];

    Box::new(Year { year: 2023, days })
}

fn day4() {
    let reader = BufReader::new(File::open("./input/2023/day4").unwrap());
    let cards = day4_parser(reader);

    println!("Part 1: {}", day4p1_logic(&cards));
    println!("Part 2: {}", day4p2_logic(cards));
}

fn day4p2_logic(mut cards: Vec<Card>) -> usize {
    for i in 0..cards.len() {
        let wins = cards[i].wins();
        for j in (i + 1)..(i + 1 + wins) {
            cards[j].copies += cards[i].copies;
        }
    }

    cards.into_iter().map(|card| card.copies).sum()
}

fn day4p1_logic(cards: &Vec<Card>) -> usize {
    cards.iter().map(|card| card.compute_points()).sum()
}

fn day4_parser(reader: impl BufRead) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        // skip the card id
        let mut tokens_iter = line.split_whitespace().skip(2);

        // parse the winning numbers
        let mut card = Card::new();
        while let Some(token) = tokens_iter.next() {
            // end of winning numbers
            if token == "|" {
                break;
            }
            card.winning.insert(token.parse().unwrap());
        }

        // parse the numbers this card has
        while let Some(token) = tokens_iter.next() {
            card.have.push(token.parse().unwrap());
        }

        cards.push(card);
    }
    cards
}

struct Card {
    pub copies: usize,
    pub have: Vec<u64>,
    pub winning: HashSet<u64>,
}

impl Card {
    pub fn new() -> Self {
        Card {
            copies: 1,
            have: Vec::new(),
            winning: HashSet::new(),
        }
    }

    pub fn compute_points(&self) -> usize {
        let wins = self.wins();
        if wins == 0 {
            0
        } else {
            1 << wins - 1
        }
    }

    pub fn wins(&self) -> usize {
        self.have
            .iter()
            .filter(|num| self.winning.contains(num))
            .count()
    }
}

fn day3() {
    let reader = BufReader::new(File::open("./input/2023/day3").unwrap());
    let result = day3p2_logic(reader);
    println!("{}", result);
}

fn day3p2_logic(reader: impl BufRead) -> u64 {
    // figure out line length
    let mut reader_iter = reader.lines().peekable();
    let line_length = reader_iter.peek().unwrap().as_ref().unwrap().len();
    let reader_iter = reader_iter.map(|x| x.unwrap());

    // set up dummy lines to be referenced at the beginning and end of iteration
    let prev_line: String = iter::repeat('.').take(line_length).collect();
    let last_line = vec![prev_line.clone()];

    let mut reader_iter = reader_iter.chain(last_line.into_iter());
    let current_line = reader_iter.next().unwrap();

    let mut gear_ratios: Vec<HashMap<usize, (u64, u32)>> = Vec::new();
    gear_ratios.push(HashMap::new());

    let mut prev: Vec<char> = prev_line.chars().collect();
    let mut current: Vec<char> = current_line.chars().collect();

    let mut line_num = 0;

    for next_line in reader_iter {
        let mut next: Vec<char> = next_line.chars().collect();

        gear_ratios.push(HashMap::new());
        // figure out which numbers to are to be included
        day3p2_find_adjascent_parts(
            &mut prev,
            &mut current,
            &mut next,
            &mut gear_ratios,
            line_num,
        );

        prev = current;
        current = next;
        line_num += 1;
    }

    gear_ratios
        .into_iter()
        .map(|hashmap| {
            let mut sum = 0;
            for (_key, (value, count)) in hashmap {
                if count == 2 {
                    sum += value;
                }
            }

            sum
        })
        .sum()
}

fn day3p2_find_adjascent_parts(
    prev: &mut [char],
    current: &mut [char],
    next: &mut [char],
    gears: &mut Vec<HashMap<usize, (u64, u32)>>,
    line_num: usize,
) {
    let mut digits: Vec<char> = Vec::new();

    for i in 0..current.len() {
        if current[i].is_ascii_digit() {
            digits.push(current[i]);
        // found end of number
        } else if !digits.is_empty() {
            let value: String = digits.iter().collect();
            let value: u64 = value.parse().unwrap();

            day3p2_check_symbol_range(
                i - digits.len(),
                i,
                [prev, current, next],
                gears,
                value,
                line_num,
            );

            digits.clear();
        }

        // found end of line
        if i == current.len() - 1 && !digits.is_empty() {
            let value: String = digits.iter().collect();
            let value: u64 = value.parse().unwrap();

            day3p2_check_symbol_range(
                i - digits.len() + 1,
                i,
                [prev, current, next],
                gears,
                value,
                line_num,
            );

            digits.clear();
        }
    }
}

fn day3p2_check_symbol_range(
    from: usize,
    mut to: usize,
    lines: [&mut [char]; 3],
    gears: &mut Vec<HashMap<usize, (u64, u32)>>,
    value: u64,
    line_num: usize,
) {
    // extend from and to by 1 if not at bounds
    let from = if from > 0 { from - 1 } else { from };
    to += 1;

    for offset in 0..lines.len() {
        for i in from..to {
            if lines[offset][i] == '*' {
                gears[line_num + offset - 1]
                    .entry(i)
                    .and_modify(|(val, count)| {
                        *val *= value;
                        *count += 1;
                    })
                    .or_insert((value, 1));
            }
        }
    }
}
fn _day3p1_logic(reader: impl BufRead) -> u64 {
    // figure out line length
    let mut reader_iter = reader.lines().peekable();
    let line_length = reader_iter.peek().unwrap().as_ref().unwrap().len();
    let reader_iter = reader_iter.map(|x| x.unwrap());

    // set up dummy lines to be referenced at the beginning and end of iteration
    let prev_line: String = iter::repeat('.').take(line_length).collect();
    let last_line = vec![prev_line.clone()];

    let mut reader_iter = reader_iter.chain(last_line.into_iter());
    let current_line = reader_iter.next().unwrap();

    let mut engine_parts: Vec<u64> = Vec::new();

    let mut prev: Vec<char> = prev_line.chars().collect();
    let mut current: Vec<char> = current_line.chars().collect();

    for next_line in reader_iter {
        let mut next: Vec<char> = next_line.chars().collect();

        // figure out which numbers to are to be included
        _day3_find_adjascent_parts(&mut prev, &mut current, &mut next, &mut engine_parts);

        prev = current;
        current = next;
    }

    engine_parts.into_iter().sum()
}

fn _day3_find_adjascent_parts(
    prev: &mut [char],
    current: &mut [char],
    next: &mut [char],
    parts: &mut Vec<u64>,
) {
    let mut digits: Vec<char> = Vec::new();

    for i in 0..current.len() {
        if current[i].is_ascii_digit() {
            digits.push(current[i]);
        // found end of number
        } else if !digits.is_empty() {
            if _day3_check_symbol_range(i - digits.len(), i, [prev, current, next]) {
                let parts_num: String = digits.iter().collect();
                let parts_num: u64 = parts_num.parse().unwrap();

                parts.push(parts_num);
                print!("\x1b[93m{}\x1b[0m", parts_num);
            } else {
                let str: String = digits.iter().collect();
                print!("{}", str);
            }

            digits.clear();
        }

        if !current[i].is_ascii_digit() {
            print!("{}", current[i])
        }

        // found end if line
        if i == current.len() - 1 && !digits.is_empty() {
            if _day3_check_symbol_range(i - digits.len() + 1, i, [prev, current, next]) {
                let parts_num: String = digits.iter().collect();
                let parts_num: u64 = parts_num.parse().unwrap();

                parts.push(parts_num);
                print!("\x1b[93m{}\x1b[0m", parts_num);
            } else {
                let str: String = digits.iter().collect();
                print!("{}", str);
            }
            digits.clear();
        }
    }

    print!("\n");
}

fn _day3_check_symbol_range(from: usize, mut to: usize, lines: [&mut [char]; 3]) -> bool {
    // extend from and to by 1 if not at bounds
    let from = if from > 0 { from - 1 } else { from };
    to += 1;

    for arr in lines {
        for i in from..to {
            if !(arr[i].is_ascii_digit()) && !(arr[i] == '.') {
                return true;
            }
        }
    }
    false
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
    use crate::years::year2023::{
        _day2p1_logic, _day3p1_logic, day1_logic, day2p2_logic, day3p2_logic, day4_parser,
        day4p1_logic, day4p2_logic,
    };

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

    #[test]
    fn day3p1_case1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(4361, _day3p1_logic(input.as_bytes()));
    }

    #[test]
    fn day3p2_case1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(467835, day3p2_logic(input.as_bytes()));
    }

    #[test]
    fn day4p1_case1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = day4_parser(input.as_bytes());

        assert_eq!(13, day4p1_logic(&cards));
    }

    #[test]
    fn day4p2_case1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = day4_parser(input.as_bytes());

        assert_eq!(30, day4p2_logic(cards));
    }
}
