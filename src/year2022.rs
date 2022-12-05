use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::collections::HashSet;
use crate::{ AdventYear, Year };

pub fn init() -> Box<dyn AdventYear> {
    let days: Vec<Box<dyn Fn()>> = vec![
        Box::new(day1), Box::new(day2), Box::new(day3), Box::new(day4)
    ];

    Box::new(Year {
        year: 2022,
        days,
    })
}

fn day4() {
    let reader = BufReader::new(File::open("./inputs/2022/day4/input").expect("can't read 2022 day3 input"));
    let elf_pairs: Vec<ElfPair> = reader.lines()
        .map(|pair| {
            let line = pair.expect("can't read 2022 day 4 input");
            let elf_ranges: Vec<&str> = line
                .split(',')
                .collect();
            assert!(elf_ranges.len() == 2, "unexpected input format");

            let mut elf_pair: Vec<Vec<usize>> = elf_ranges.iter()
                .map(|elf_range| { 
                    let range: Vec<usize> = elf_range.split('-')
                        .map(|range_end| { range_end.parse::<usize>().expect("unable to parse int from range") })
                        .collect();
                    range
                })
                .collect();

            assert!(elf_pair.len() == 2, "invalid number of elves parsed from an elf pair");
            let elf2 = elf_pair.pop().unwrap();
            let elf1 = elf_pair.pop().unwrap();
            ElfPair::new(elf1, elf2)
        })
        .collect();

    let num_overlapping = elf_pairs.iter()
        .filter(|pair| pair.overlapping())
        .count();

    println!("Fully overlapping assignment pairs: {}", num_overlapping);
}

#[derive(Debug)]
struct ElfPair {
    elf1: Vec<usize>,
    elf2: Vec<usize>,
}

impl ElfPair {
    fn new(elf1: Vec<usize>, elf2: Vec<usize>) -> ElfPair {
        assert!(elf1.len() == 2, "invalid range assigned to elf, likely parsing error");
        assert!(elf2.len() == 2, "invalid range assigned to elf, likely parsing error");

        ElfPair {
            elf1,
            elf2,
        }
    }

    // determine whether one range contains the other (bidirectional)
    fn overlapping(&self) -> bool {
        (self.elf1[0] <= self.elf2[0] && self.elf1[1] >= self.elf2[1]) ||
            (self.elf2[0] <= self.elf1[0] && self.elf2[1] >= self.elf1[1])
    }
}

fn day3() {
    let reader = BufReader::new(File::open("./inputs/2022/day3/input").expect("can't read 2022 day3 input"));
    let mut rucksacks: Vec<Rucksack> = reader.lines()
        .map(|line| { Rucksack::new(line.expect("io error on line")) })
        .collect();

    let rucksack_priority_sum: usize = rucksacks.iter_mut()
        .map(|rucksack| { rucksack.common_priority() })
        .sum();
    println!("Sum of priorities: {}", rucksack_priority_sum);


    // check that there are only complete groups
    let group_size = 3;
    assert!(rucksacks.len() % group_size == 0, "Incomplete groups found in input");

    let mut badges: Vec<char> = vec![];
    for i in (0..rucksacks.len()).step_by(group_size) {
        let group_intersection = &(&rucksacks[i].item_set & &rucksacks[i + 1].item_set) & &rucksacks[i + 2].item_set;
        assert!(group_intersection.len() == 1, "multiple badge options found for a single group");
        badges.push(group_intersection.into_iter().next().unwrap());
    }

    let group_priority_sum: usize = badges.iter()
        .map(|badge| Rucksack::compute_item_priority(*badge))
        .sum();

    println!("Sum of badge priorities: {}", group_priority_sum);
}

struct Rucksack {
    total: Box<String>,
    pub item_set: HashSet<char>,
    midpoint: usize,
    common_item: Option<char>,
    common_priority: Option<usize>,
}

impl Rucksack {
    pub fn new(inventory: String) -> Rucksack {
        // check function invariants
        assert!(inventory.len() % 2 == 0);
        assert!(inventory.is_ascii());
        assert!(inventory.chars().all(char::is_alphabetic));

        let midpoint = inventory.len() / 2;

        let item_set = HashSet::from_iter(inventory.chars());

        Rucksack {
            total: Box::new(inventory),
            item_set,
            midpoint,
            common_item: None,
            common_priority: None,
        }
    }

    pub fn common_item(&mut self) -> char {
        let compartment1 = &self.total[..self.midpoint];
        let compartment2 = &self.total[self.midpoint..];

        let mut common_item = None;
        for item in compartment1.chars() {
            if compartment2.contains(item) {
                match common_item {
                    Some(x) => assert!(x == item, "multiple matching items in Rucksack"),
                    None => common_item = Some(item),
                }
            }
        }

        if let None = common_item { panic!("Unable to find matching item in Rucksack") }

        self.common_item = common_item;
        common_item.unwrap()
    }

    pub fn common_priority(&mut self) -> usize {
        let common_item = match self.common_item {
            None => self.common_item(),
            Some(item) => item,
        };

        self.common_priority = Some(Rucksack::compute_item_priority(common_item));
        self.common_priority.unwrap()
    }

    fn compute_item_priority(item: char) -> usize {
        let upper_case_offset = 38;
        let lower_case_offset = 96;

        let mut buff = [0; 4];
        item.encode_utf8(&mut buff);
        if item.is_uppercase() {
            usize::from(buff[0]) - upper_case_offset
        }
        else {
            usize::from(buff[0]) - lower_case_offset
        }
    }
}

fn day2() {
    let reader = BufReader::new(File::open("./inputs/2022/day2/input").expect("unable to read input file for 2022 day2"));
    
    let rounds: Vec<(RPSRound, RPSRound)> = reader.lines()
        .map(|round| {
            let line = round.expect("unable to read line");
            let mut char_iter = line.chars();

            // parse opponent move
            let opponent = match char_iter.next().unwrap() {
                'A' => RPS::Rock,
                'B' => RPS::Paper,
                'C' => RPS::Scissors,
                _ => panic!("unexpected first symbol"),
            };

            // skip whitespace character
            char_iter.next().unwrap();

            // parse my move for question 1
            let my_char = char_iter.next().unwrap();
            let me_part1 = match my_char {
                'X' => RPS::Rock,
                'Y' => RPS::Paper,
                'Z' => RPS::Scissors,
                _ => panic!("unexpected third symbol"),
            };


            // parse my move for question 2
            let me_part2 =  match opponent {
                RPS::Rock => {
                    match my_char {
                        'X' => RPS::Scissors,
                        'Y' => RPS::Rock,
                        'Z' => RPS::Paper,
                        _ => panic!("unexpected third symbol"),
                    }
                },
                RPS::Paper => {
                    match my_char {
                        'X' => RPS::Rock,
                        'Y' => RPS::Paper,
                        'Z' => RPS::Scissors,
                        _ => panic!("unexpected third symbol"),
                    }
                },
                RPS::Scissors => {
                    match my_char {
                        'X' => RPS::Paper,
                        'Y' => RPS::Scissors,
                        'Z' => RPS::Rock,
                        _ => panic!("unexpected third symbol"),
                    } 
                }
            };

            // return a tuple formatted (part 1 round, part 2 round)
            (RPSRound::new(opponent, me_part1), RPSRound::new(opponent, me_part2))
        })
        .collect();

    let (mut total_part1, mut total_part2) = (0, 0);

    // sum up rounds for parts 1 and 2
    for round in rounds {
        total_part1 += round.0.score().1;
        total_part2 += round.1.score().1;
    }

    println!("Part 1 total score: {}", total_part1);
    println!("Part 2 total score: {}", total_part2);
}


#[derive(Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

struct RPSRound {
    opponent: RPS,
    me: RPS,
}

impl RPSRound {
    pub fn new(opponent: RPS, me: RPS) -> RPSRound {
        RPSRound {
            opponent,
            me,
        }
    }

    pub fn score(&self) -> (i32, i32) {
        let (mut opponent_score, mut my_score) = RPSRound::outcome_score(self.opponent, self.me);
        opponent_score += RPSRound::symbol_score(self.opponent);
        my_score += RPSRound::symbol_score(self.me);

        (opponent_score, my_score)

    }

    fn symbol_score(symbol: RPS) -> i32 {
        match symbol {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn outcome_score(player1: RPS, player2: RPS) -> (i32, i32) {
        match player1 {
            RPS::Rock => {
                match player2 {
                    RPS::Rock => (3, 3),  
                    RPS::Paper => (0, 6), 
                    RPS::Scissors => (6, 0),
                }
            },
            RPS::Paper => {
                match player2 {
                    RPS::Rock => (6, 0),
                    RPS::Paper => (3, 3),
                    RPS::Scissors => (0, 6),
                }
            },
            RPS::Scissors => {
                match player2 {
                    RPS::Rock => (0, 6),
                    RPS::Paper => (6, 0),
                    RPS::Scissors => (3, 3),
                }
            }
        }
    }
}

fn day1() {
    let mut total_calories: Vec<usize> = vec![];

    let lines_iter = io::BufReader::new(
        File::open("./inputs/2022/day1/input").expect("can't open input file")
        ).lines();


    let mut elf_total: usize = 0;
    for line in lines_iter {
        if let Ok(x) = line {
            if x.trim().is_empty() {
                total_calories.push(elf_total);
                elf_total = 0;
            }
            else {
                let single_item: usize = x.parse().expect("unable to parse a valid usize from input");
                elf_total += single_item;
            }
        }
    }

    total_calories.sort_unstable();

    println!("Highest Calories: {}", total_calories.last().unwrap());

    let mut num: usize = 0;
    let top3_calories: usize = total_calories
        .iter()
        .rev()
        .filter(|_| {
            num += 1;
            num <= 3
        })
        .sum();
    
    println!("Top 3 Calories: {}", top3_calories);

}


