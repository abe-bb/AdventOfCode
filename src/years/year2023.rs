use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use itertools::Itertools;

use crate::{AdventYear, Year};

pub fn init() -> Box<dyn AdventYear> {
    let days: Vec<Box<dyn Fn()>> = vec![
        Box::new(day1),
        Box::new(day2),
        Box::new(day3),
        Box::new(day4),
        Box::new(day5),
        Box::new(day6),
        Box::new(day7),
        Box::new(day8),
    ];

    Box::new(Year { year: 2023, days })
}

fn day8() {
    let reader = BufReader::new(File::open("./input/2023/day8").unwrap());
    let (instructions, mut adjacency) = day8_parse(reader);

    println!(
        "Part 1 Num Steps: {}",
        day8_p1(&instructions, &mut adjacency)
    );

    println!(
        "Part 2 Num Steps: {}",
        day8_p2(&instructions, &mut adjacency)
    );
}

fn day8_p2(instructions: &str, adjacency: &mut AdjacencyGraph) -> u64 {
    let nodes = adjacency
        .adjacency
        .iter()
        .filter_map(|(key, _)| -> Option<&str> {
            if key.as_bytes()[2] as char == 'A' {
                Some(key)
            } else {
                None
            }
        })
        .collect_vec();

    let mut step_counts = nodes
        .iter()
        .map(|start_node| {
            let mut node = *start_node;
            let mut index = 0;
            let mut steps = 0;

            while node.as_bytes()[2] as char != 'Z' {
                let left = instructions.as_bytes()[index] as char == 'L';
                node = adjacency.turn(node, left);
                steps += 1;

                // increment instruction index
                index = (index + 1) % instructions.len();
            }

            steps
        })
        .collect_vec();

    // sort descending
    step_counts.sort_unstable_by(|a, b| a.cmp(b).reverse());

    // greatest common factor
    let gcf = instructions.len();
    // least common multiple
    let mut lcm = step_counts.pop().unwrap() * step_counts.pop().unwrap() / gcf;

    for steps in step_counts {
        lcm = lcm * steps / gcf;
    }

    lcm as u64
}

fn day8_p1(instructions: &str, adjacency: &mut AdjacencyGraph) -> u64 {
    let mut node = "AAA";
    let mut index = 0;
    let mut steps = 0;

    while node != "ZZZ" {
        let left = instructions.as_bytes()[index] as char == 'L';
        node = adjacency.turn(node, left);
        steps += 1;

        // increment instruction index
        index = (index + 1) % instructions.len();
    }

    steps
}

fn day8_parse(reader: impl BufRead) -> (String, AdjacencyGraph) {
    let mut lines = reader.lines().map(|x| x.unwrap());
    // read directions
    let directions = lines.next().unwrap();
    lines.next();

    // read nodes
    let adjacency = lines.fold(HashMap::new(), |mut adjacency, line| {
        let mut nodes: Vec<String> = line
            .split([' ', ',', '(', ')'])
            .filter_map(|x| {
                if x.len() == 3 {
                    Some(x.to_owned())
                } else {
                    None
                }
            })
            .collect_vec();

        let right = nodes.pop().unwrap();
        let left = nodes.pop().unwrap();
        let node = nodes.pop().unwrap();

        adjacency.insert(node, (left, right));
        adjacency
    });

    (directions, AdjacencyGraph { adjacency })
}

struct AdjacencyGraph {
    pub adjacency: HashMap<String, (String, String)>,
}

impl AdjacencyGraph {
    pub fn turn<'a, 'b>(&'a self, node: &'b str, left: bool) -> &'a str {
        let choice = self.adjacency.get(node).unwrap();

        if left {
            &choice.0
        } else {
            &choice.1
        }
    }
}

fn day7() {
    let reader = BufReader::new(File::open("./input/2023/day7").unwrap());
    let mut bids = day7_parse(reader);
    // sort by hands with weakest hand first
    bids.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    println!("Part 2: Winnings = {}", day7p2_logic(&bids));
}

fn day7p2_logic(bids: &Vec<(Hand, u64)>) -> u64 {
    bids.iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u64 + 1))
        .sum()
}

fn day7_parse(reader: impl BufRead) -> Vec<(Hand, u64)> {
    // awww yisss, single iter chain parser
    // shout out to itertools, I just found you and I already love you
    reader
        .lines()
        .map(|x| x.unwrap())
        .flat_map(|x| x.split_whitespace().map(|x| x.to_owned()).collect_vec())
        .tuples()
        .map(|(hand, bid)| (Hand::try_from(hand.as_str()).unwrap(), bid.parse().unwrap()))
        .collect()
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    strength: u8,
}

impl Hand {
    fn strength(cards: &[u8; 5]) -> u8 {
        let mut freq = cards
            .iter()
            // build hashmap with entry frequencies
            .counts()
            .into_iter()
            // convert key and count to both be u8
            .map(|(key, count)| (*key, count as u8))
            // sort by frequency
            .sorted_unstable_by(|a, b| a.1.cmp(&b.1).reverse())
            .collect_vec();

        // process wilds
        let mut j = 0;
        for card in freq.iter_mut() {
            // found joker freqency
            if card.0 == 0 {
                j = card.1;
                card.1 = 0;
                break;
            }
        }

        // re sort with wild frequency set to 0
        freq.sort_unstable_by(|a, b| a.1.cmp(&b.1).reverse());
        // add wild frequency to most frequent card
        freq[0].1 += j;

        // find hand strength
        match freq[0].1 {
            5 => 6, // five of a kind
            4 => 5, // four of a kind
            3 => match freq[1].1 {
                2 => 4, // full house
                1 => 3, // three of a kind
                _ => unreachable!(),
            },
            2 => match freq[1].1 {
                2 => 2, // two pair
                1 => 1, // one pair
                _ => unreachable!(),
            },
            1 => 0, // high card
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Sort hands first by strength, then by individual card strength
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut result = self.strength.cmp(&other.strength);

        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if result != Ordering::Equal {
                break;
            }

            result = card.cmp(other_card);
        }

        result
    }
}

impl TryFrom<&str> for Hand {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cards: Vec<u8> = value
            .chars()
            .map(|c| match c {
                '2' => Ok(2),
                '3' => Ok(3),
                '4' => Ok(4),
                '5' => Ok(5),
                '6' => Ok(6),
                '7' => Ok(7),
                '8' => Ok(8),
                '9' => Ok(9),
                'T' => Ok(10),
                // J's are wild, score lower than every other card
                'J' => Ok(0),
                'Q' => Ok(12),
                'K' => Ok(13),
                'A' => Ok(14),
                _ => Err("Unexpected character"),
            })
            .collect::<Result<Vec<_>, _>>()?;

        if cards.len() != 5 {
            return Err("Hands must be 5 cards");
        }

        let cards = cards.try_into().unwrap();

        Ok(Hand {
            strength: Hand::strength(&cards),
            cards,
        })
    }
}

fn day6() {
    let reader = BufReader::new(File::open("./input/2023/day6").unwrap());
    let races = day6_parse(reader);
    let p1: u64 = races
        .iter()
        .map(|race| race.ways_to_win().unwrap())
        .product();

    println!("Part 1: {}", p1);

    let p2_time: String = races.iter().map(|race| race.time.to_string()).collect();
    let p2_record: String = races.iter().map(|race| race.record.to_string()).collect();
    let p2_time: u64 = p2_time.parse().unwrap();
    let p2_record: u64 = p2_record.parse().unwrap();
    let p2_race = Race::new(p2_time, p2_record);
    let p2_w2win = p2_race.ways_to_win().unwrap();

    println!("Part 2: {}", p2_w2win);
}

fn day6_parse(reader: impl BufRead) -> Vec<Race> {
    let mut lines = reader.lines().map(|x| x.unwrap());

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let records: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    times
        .into_iter()
        .zip(records.into_iter())
        .map(|(time, record)| Race::new(time, record))
        .collect()
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn new(time: u64, record: u64) -> Self {
        Race { time, record }
    }

    pub fn ways_to_win(&self) -> Option<u64> {
        if let Some((lower, upper)) = self.record_button_hold_times() {
            let mut exact_count = 0;
            if lower.ceil() == lower {
                exact_count += 1;
            }
            if upper.floor() == upper {
                exact_count += 1;
            }

            Some(upper.floor() as u64 - lower.ceil() as u64 + 1 - exact_count)
        } else {
            None
        }
    }

    pub fn record_button_hold_times(&self) -> Option<(f64, f64)> {
        let time = self.time as f64;
        let record = self.record as f64;

        // half of quadratic formula
        let upper = (-time - (time.powi(2) - 4. * record).sqrt()) / -2.;
        // other half of quadratic formula
        let lower = (-time + (time.powi(2) - 4. * record).sqrt()) / -2.;

        if upper.is_nan() || lower.is_nan() {
            None
        } else {
            Some((lower, upper))
        }
    }
}

fn day5() {
    let reader = BufReader::new(File::open("./input/2023/day5").unwrap());
    let mut almanic = day5_parse(reader);
    let mut locations: Vec<u64> = almanic.find_locations_p1();
    locations.sort_unstable();
    println!("Nearest Location Part 1: {}", locations.first().unwrap());

    let mut location_ranges = almanic.find_locations();
    location_ranges.sort_unstable_by(|span1, span2| span1.start.cmp(&span2.start));
    println!(
        "Nearest Location Part 2: {}",
        location_ranges.first().unwrap().start
    );
}

fn day5_parse(reader: impl BufRead) -> Almanac {
    let mut line_iter = reader.lines().map(|x| x.unwrap());

    // parse seeds
    let seeds: Vec<u64> = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|seed| seed.parse().ok())
        .collect();
    // skip first empty line
    line_iter.next();

    let mut mappings: Vec<Mapping> = Vec::new();

    while let Some(line) = line_iter.next() {
        // look for mapping headers
        let mut mapping = line.split(' ').next().unwrap().split('-');
        let from: String = mapping.next().unwrap().to_owned();
        mapping.next();
        let to: String = mapping.next().unwrap().to_owned();

        let mapping = day5_parse_mappings(&mut line_iter, from, to);
        mappings.push(mapping);
    }

    Almanac { seeds, mappings }
}

fn day5_parse_mappings<T>(line_iter: &mut T, from: String, to: String) -> Mapping
where
    T: Iterator<Item = String>,
{
    let mut mappings: Vec<[u64; 3]> = Vec::new();
    // look for mappings
    while let Some(line) = line_iter.next() {
        // found end of current mapping, break to outer loop
        if line.is_empty() {
            break;
        }

        // parse the integers from the line into an arrayinto an array
        let map_range: [u64; 3] = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap_or_else(|_| panic!("expected 3 numbers per line\n{}", line));
        mappings.push(map_range);
    }

    Mapping {
        from,
        to,
        mappings,
        sorted: false,
    }
}

struct Span {
    pub start: u64,
    pub range: u64,
}

impl Span {
    pub fn new(start: u64, range: u64) -> Span {
        Span { start, range }
    }

    // splits this span
    // panics if impossible to create two valid spans
    pub fn split_at(&mut self, at: u64) -> Span {
        if at <= self.start || at >= self.start + self.range {
            panic!("Unable to split into two valid spans");
        }

        let old_range = at - self.start;
        let new_range = self.range - old_range;

        self.range = old_range;

        Span {
            start: at,
            range: new_range,
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("{}--{}", self.start, self.start + self.range)
    }
}

#[allow(dead_code)]
struct Almanac {
    pub seeds: Vec<u64>,
    pub mappings: Vec<Mapping>,
}

impl Almanac {
    pub fn find_locations(&mut self) -> Vec<Span> {
        // build process queue, starting by interpreting seeds as spans
        let mut process_queue: Vec<(usize, Span)> = self
            .seeds
            .chunks(2)
            .map(|slice| (0, Span::new(slice[0], slice[1])))
            .collect();

        assert!(!process_queue.is_empty());

        let mut results = Vec::new();
        while let Some((index, span)) = process_queue.pop() {
            // span was mapped through last mapping
            // add to results
            if index == self.mappings.len() {
                results.push(span);
                continue;
            }

            // map current span, push resulting spans to process queue
            let mapped = self.mappings[index].map_range(span);
            process_queue.extend(mapped.into_iter().map(|x| (index + 1, x)));
        }

        results
    }
    pub fn find_locations_p1(&mut self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed| {
                let mut input: u64 = *seed;
                for map in self.mappings.iter_mut() {
                    input = map.map(input);
                }
                input
            })
            .collect()
    }
}

#[allow(dead_code)]
struct Mapping {
    from: String,
    to: String,
    mappings: Vec<[u64; 3]>,
    sorted: bool,
}

#[allow(dead_code)]
impl Mapping {
    pub fn new(from: String, to: String) -> Mapping {
        Mapping {
            from,
            to,
            mappings: Vec::new(),
            sorted: true,
        }
    }

    pub fn add_range(&mut self, from_num: u64, to_num: u64, range: u64) {
        self.sorted = false;
        self.mappings.push([to_num, from_num, range]);
    }

    pub fn map(&mut self, from: u64) -> u64 {
        self.sort_mappings();

        // iterate over mappings looking for matching range, return "from" if no mapping found
        for mapping in self.mappings.iter() {
            // sorted mappings, so no mappings found further along
            if mapping[1] > from {
                break;
            }

            if mapping[1] <= from && (mapping[1] + mapping[2]) > from {
                let offset = from - mapping[1];

                return mapping[0] + offset;
            }
        }
        from
    }

    pub fn map_range(&mut self, from: Span) -> Vec<Span> {
        self.sort_mappings();
        let mut to = Vec::new();

        let mut span_opt = Some(from);
        for range in self.mappings.iter() {
            let mut span = span_opt.take().unwrap();
            // span above map range, gg go next
            if range[1] + range[2] <= span.start {
                span_opt = Some(span);
                continue;
            // span below map range, no explicit mappings found, break
            } else if span.start + span.range <= range[1] {
                span_opt = Some(span);
                break;
            }

            // check for span area below current range and map it implicitly
            if span.start < range[1] {
                let above = span.split_at(range[1]);
                to.push(span);
                span = above;
            }

            let mut next = None;
            // check for span area above current range and save it to for next iteration
            if span.start + span.range > range[1] + range[2] {
                next = Some(span.split_at(range[1] + range[2]));
            }

            // map span and push to results
            let offset = span.start - range[1];
            span.start = range[0] + offset;
            to.push(span);

            // still more span to map, continue
            if let Some(span) = next {
                span_opt = Some(span);
                continue;
            // no more mapping to do, we're done
            } else {
                break;
            }
        }

        // implicitly map any remaining span
        if let Some(span) = span_opt {
            to.push(span);
        }

        to
    }

    /// sort by from range then range
    fn sort_mappings(&mut self) {
        // sort if not sorted
        if !self.sorted {
            self.mappings.sort_unstable_by(|map1, map2| {
                if map1[1] == map2[1] {
                    map1[2].cmp(&map2[2])
                } else {
                    map1[1].cmp(&map2[1])
                }
            });
            self.sorted = true;
        }
    }
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
    use super::*;

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

    #[test]
    fn day5p1_case1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let mut almanac: Almanac = day5_parse(input.as_bytes());
        let mut locations = almanac.find_locations_p1();
        locations.sort_unstable();

        assert_eq!(35, *locations.first().unwrap());
    }

    #[test]
    fn day5p2_case1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let mut almanac: Almanac = day5_parse(input.as_bytes());
        let mut location_ranges = almanac.find_locations();
        assert!(!location_ranges.is_empty());

        location_ranges.sort_unstable_by(|span1, span2| span1.start.cmp(&span2.start));
        assert_eq!(46, location_ranges.first().unwrap().start);
    }

    #[test]
    fn day6p1_case1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let races = day6_parse(input.as_bytes());
        let p1: u64 = races
            .iter()
            .map(|race| {
                let w2win = race.ways_to_win().unwrap();
                println!("{}", w2win);
                w2win
            })
            .product();
        assert_eq!(288, p1);
    }

    #[test]
    fn day7_hand_ordering() {
        let hand1 = Hand::try_from("QQQJA").unwrap();
        let hand2 = Hand::try_from("KTJJT").unwrap();

        println!(
            "QQQJA strength {}; KTJJT strength {}",
            hand1.strength, hand2.strength
        );
        assert!(hand1 < hand2);
    }

    #[test]
    fn day7p2_case1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut bids = day7_parse(input.as_bytes());
        bids.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(5905, day7p2_logic(&bids))
    }

    #[test]
    fn day8p1_case1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let (instructions, mut adjacency) = day8_parse(input.as_bytes());
        assert_eq!(2, day8_p1(&instructions, &mut adjacency));
    }
}
