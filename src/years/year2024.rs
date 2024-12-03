use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::{AdventYear, Year};

pub fn init() -> Box<dyn AdventYear> {
    let days: Vec<Box<dyn Fn()>> = vec![Box::new(day1)];

    Box::new(Year { year: 2024, days })
}

fn day1() {
    let reader = BufReader::new(File::open("./input/2024/day1").unwrap());
    let (mut list1, mut list2) = day1_parse(reader);
    println!("Part 1: {}", day1p1_logic(&mut list1, &mut list2));
    println!("Part 2: {}", day1p2_logic(&list1, &list2));
}

fn day1_parse(reader: impl BufRead) -> (Vec<i32>, Vec<i32>) {
    reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .next_tuple::<(i32, i32)>()
                .unwrap()
        })
        .unzip()
}

fn day1p1_logic(list1: &mut Vec<i32>, list2: &mut Vec<i32>) -> i32 {
    list1.sort_unstable();
    list2.sort_unstable();

    // calculate the difference between each number on the right and left
    // and sum them all together
    list1
        .iter()
        .zip(list2.iter())
        .map(|x| (x.0 - x.1).abs())
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn day1p2_logic(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut rep_counter: HashMap<i32, i32> = HashMap::new();

    // count the repititions of each number
    for num in list2.iter() {
        *rep_counter.entry(*num).or_insert(0) += 1;
    }

    // multiply the number from the left list, by the frequency of that number on the right
    // and then sum them all together
    list1
        .iter()
        .filter_map(|x| rep_counter.get(x).and_then(|y| Some(x * y)))
        .reduce(|acc, e| acc + e)
        .unwrap()
}
