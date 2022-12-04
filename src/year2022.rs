use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use crate::{ AdventYear, Year };

pub fn init() -> Box<dyn AdventYear> {
    let days: Vec<Box<dyn Fn()>> = vec![
        Box::new(day1), Box::new(day2)
    ];

    Box::new(Year {
        year: 2022,
        days,
    })
}


fn day2() {
    let reader = BufReader::new(File::open("./inputs/2022/day2/input").expect("unable to read input file for 2022 day2"));
    
    let rounds: Vec<RPSRound> = reader.lines()
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

            // parse my move
            let me = match char_iter.next().unwrap() {
                'X' => RPS::Rock,
                'Y' => RPS::Paper,
                'Z' => RPS::Scissors,
                _ => panic!("unexpected third symbol"),
            };

            RPSRound::new(opponent, me)
        })
        .collect();

        let my_total: i32 = rounds.iter()
            .map(|round| round.score().1)
            .sum();

        println!("My total: {}", my_total);
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


