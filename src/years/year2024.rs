use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    iter::repeat_n,
};

use itertools::Itertools;
use nalgebra::{Const, DMatrix, Dyn, Matrix, VecStorage, ViewStorage};
use regex::Regex;

use crate::{AdventYear, Year};

pub fn init() -> Box<dyn AdventYear> {
    let days: Vec<Box<dyn Fn()>> = vec![
        Box::new(day1),
        Box::new(day2),
        Box::new(day3),
        Box::new(day4),
    ];

    Box::new(Year { year: 2024, days })
}

fn day4() {
    let reader = BufReader::new(File::open("./input/2024/day4").unwrap());
    let mut data = day4_parse(reader);
    println!("Part 1: {}", day4p1_logic(&mut data));
    println!("Part 2: {}", day4p2_logic(&mut data));
}

fn day4p2_logic(data: &mut Matrix<i32, Dyn, Dyn, VecStorage<i32, Dyn, Dyn>>) -> usize {
    let mut cycle: Vec<i32> = vec![0; data.ncols() - 1];
    *cycle.last_mut().unwrap() = 1;
    let mut reflect = DMatrix::from_row_iterator(
        data.nrows(),
        data.ncols(),
        repeat_n(0i32, 1).chain(cycle.into_iter().cycle()),
    );
    *reflect
        .get_mut((reflect.nrows() - 1, reflect.ncols() - 1))
        .unwrap() = 0;
    let mut counter: usize = 0;
    for _ in 0..4 {
        let (r, c) = data.shape();
        for row in 0..(r - 2) {
            for col in 0..(c - 2) {
                if *data.get((row, col)).unwrap() == 2 {
                    counter += day4_check_x_mas(data.view((row, col), (3, 3)));
                }
            }
        }

        let new_data = (&reflect) * data.transpose();
        *data = new_data;
    }

    counter
}

fn day4_check_x_mas(
    view: Matrix<i32, Dyn, Dyn, ViewStorage<'_, i32, Dyn, Dyn, Const<1>, Dyn>>,
) -> usize {
    // check for x-mas
    if *view.get((0, 0)).unwrap() == 2
        && *view.get((0, 2)).unwrap() == 2
        && *view.get((1, 1)).unwrap() == 3
        && *view.get((2, 0)).unwrap() == 4
        && *view.get((2, 2)).unwrap() == 4
    {
        1
    } else {
        0
    }
}

fn day4p1_logic(data: &mut Matrix<i32, Dyn, Dyn, VecStorage<i32, Dyn, Dyn>>) -> usize {
    let mut cycle: Vec<i32> = vec![0; data.ncols() - 1];
    *cycle.last_mut().unwrap() = 1;
    let mut reflect = DMatrix::from_row_iterator(
        data.nrows(),
        data.ncols(),
        repeat_n(0i32, 1).chain(cycle.into_iter().cycle()),
    );
    *reflect
        .get_mut((reflect.nrows() - 1, reflect.ncols() - 1))
        .unwrap() = 0;
    let mut counter: usize = 0;
    for _ in 0..4 {
        let (r, c) = data.shape();
        for row in 0..(r) {
            for col in 0..(c - 3) {
                if *data.get((row, col)).unwrap() == 1 {
                    let check_diag = row <= (r - 4);
                    if check_diag {
                        counter += day4_check_xmas(data.view((row, col), (4, 4)), check_diag);
                    } else {
                        counter += day4_check_xmas(data.view((row, col), (1, 4)), check_diag);
                    }
                }
            }
        }

        let new_data = (&reflect) * data.transpose();
        *data = new_data;
    }

    counter
}

fn day4_check_xmas(
    view: Matrix<i32, Dyn, Dyn, ViewStorage<'_, i32, Dyn, Dyn, Const<1>, Dyn>>,
    check_diag: bool,
) -> usize {
    let mut counter = 0;
    // check straight ahead
    if *view.get((0, 0)).unwrap() == 1
        && *view.get((0, 1)).unwrap() == 2
        && *view.get((0, 2)).unwrap() == 3
        && *view.get((0, 3)).unwrap() == 4
    {
        counter += 1;
    }
    // check diagonal
    if check_diag
        && *view.get((0, 0)).unwrap() == 1
        && *view.get((1, 1)).unwrap() == 2
        && *view.get((2, 2)).unwrap() == 3
        && *view.get((3, 3)).unwrap() == 4
    {
        counter += 1;
    }

    counter
}

fn day4_parse(mut reader: impl BufRead) -> DMatrix<i32> {
    let mut data: String = String::new();
    reader
        .read_to_string(&mut data)
        .expect("Failed to read input data");
    let row_length = data.lines().next().unwrap().len();
    let col_length = data.lines().count();

    DMatrix::from_row_iterator(
        col_length,
        row_length,
        data.chars().filter_map(|c| match c {
            'X' => Some(1),
            'M' => Some(2),
            'A' => Some(3),
            'S' => Some(4),
            '\n' => None,
            _ => Some(0),
        }),
    )
}

fn day3() {
    let mut input: String = String::new();
    File::read_to_string(&mut File::open("./input/2024/day3").unwrap(), &mut input)
        .expect("Failed to read input to string");

    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();

    let mut p1_result = 0;
    let mut p2_result = 0;
    let mut enable = true;

    for capture in re.captures_iter(&input) {
        match &capture.get(0).unwrap().as_str()[..3] {
            "do(" => enable = true,
            "don" => enable = false,
            "mul" => {
                let slice = &capture.get(0).unwrap().as_str()[4..];
                let slice = &slice[..(slice.len() - 1)];
                let (left, right) = slice
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .next_tuple()
                    .unwrap();
                let mult = left * right;

                p1_result += mult;
                if enable {
                    p2_result += mult;
                }
            }
            _ => {
                panic!("Unexpected capture group!")
            }
        }
    }

    println!("Part 1: {}", p1_result);
    println!("Part 2: {}", p2_result);
}

fn day2() {
    let reader = BufReader::new(File::open("./input/2024/day2").unwrap());
    let reports = day2_parse(reader);
    println!("Part 1: {}", day2p1_logic(&reports));
    println!("Part 2: {}", day2p2_logic(&reports));
}

fn day2_parse(reader: impl BufRead) -> Vec<Vec<i32>> {
    reader
        .lines()
        .map(|x| {
            // parse each whitespace separated character into an integer and collect into a vector
            x.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        // collect the vectors into a vector
        .collect_vec()
}

fn day2p1_logic(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .filter_map(
            // safe reports return Some(()), unsafe reports return None
            |report| {
                if report.len() == 0 {
                    return None;
                }
                if report.len() == 1 {
                    return Some(());
                }

                let init_diff = report[0] - report[1];
                // check initial safety
                if init_diff == 0 || init_diff.abs() > 3 {
                    return None;
                }

                let mut sliding_window = report.windows(2);
                // skip first pair
                sliding_window.next();

                for window in sliding_window {
                    let diff = window[0] - window[1];
                    // check that the difference is within parameters
                    if diff * init_diff <= 0 || diff.abs() > 3 {
                        return None;
                    }
                }

                // if all levels are within parameters, report safe
                Some(())
            },
        )
        .count()
        .try_into()
        .unwrap()
}

fn day2p2_logic(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .filter_map(
            // safe reports return Some(()), unsafe reports return None
            |report| {
                if report.len() == 0 {
                    return None;
                }
                if report.len() == 1 {
                    return Some(());
                }

                let init_diff = report[0] - report[1];

                let mut prev = report[0];

                for i in 1..(report.len()) {
                    let level = report[i];
                    let diff = prev - level;

                    if diff * init_diff <= 0 || diff.abs() > 3 {
                        if day2_check_report(&report[..i], &report[(i + 1)..])
                            || day2_check_report(&report[..(i - 1)], &report[i..])
                        {
                            return Some(());
                        } else {
                            return None;
                        }
                    }

                    prev = level;
                }

                // if all levels are within parameters, report safe
                Some(())
            },
        )
        .count()
        .try_into()
        .unwrap()
}

fn day2_check_report(report1: &[i32], report2: &[i32]) -> bool {
    let report = [report1, report2].concat();

    let mut prev = report[0];
    let init_diff = report[0] - report[1];

    for level in report[1..].iter() {
        let diff = prev - level;

        if diff * init_diff <= 0 || diff.abs() > 3 {
            return false;
        }

        prev = *level;
    }

    true
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

#[cfg(test)]
mod test {
    use crate::years::year2024::{day2p1_logic, day2p2_logic};

    use super::day2_parse;

    #[test]
    fn day2p1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let reports = day2_parse(input.as_bytes());
        assert_eq!(2, day2p1_logic(&reports));
        assert_eq!(4, day2p2_logic(&reports));
    }
}
