use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut year = 0;
    let mut day = 0;

    // parse year and day if available
    if args.len() >= 2 {
        year = args[1].parse().expect(&format!("{} is not a valid year", args[1]));
    }
    if args.len() >= 3 {
        day = args[2].parse().expect(&format!("{} is not a valid day", args[2]));
    }

    let advent_manager = advent_of_code::init();

    // run solution for a day
    advent_manager.solve_day(year, day);
}
