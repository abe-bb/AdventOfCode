use std::fs::File;
use std::io::{ self, BufRead };
use crate::{ AdventYear, Year };

pub fn init() -> Box<dyn AdventYear> {
    Box::new(Year {
        year: 2022,
        days: vec![Box::new(day1)],
    })
}

pub fn day1() {
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


