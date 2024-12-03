pub mod years;

pub fn init() -> AdventManager {
    let mut years = vec![];

    years.push(years::year2022::init());
    years.push(years::year2023::init());
    years.push(years::year2024::init());

    AdventManager::new(years)
}

pub trait AdventYear {
    fn year(&self) -> usize;
    fn solve(&self, day: usize);
    fn solve_latest(&self);
}

struct Year {
    year: usize,
    days: Vec<Box<dyn Fn()>>,
}

impl AdventYear for Year {
    fn year(&self) -> usize {
        self.year
    }

    fn solve(&self, day: usize) {
        // call latest if day is set to 0
        if day == 0 {
            return self.solve_latest();
        }

        assert!(
            day > 0 && day < self.days.len(),
            "day {} is unimplemented. days {}-{} are implemented for selected year. 0 for latest",
            day,
            1,
            self.days.len()
        );

        if day >= self.days.len() {
            println!("unimplemented");
        }

        self.days[day - 1]()
    }

    fn solve_latest(&self) {
        if self.days.len() == 0 {
            println!("unimplemented");
            return;
        }

        self.days.last().unwrap()()
    }
}

pub struct AdventManager {
    years: Vec<Box<dyn AdventYear>>,
}

impl AdventManager {
    /// Constructs a new AdventManager
    ///
    /// # Invariants
    /// `years` must be a sequential list of AdventYear trait objects
    pub fn new(years: Vec<Box<dyn AdventYear>>) -> AdventManager {
        // check function invariants
        let mut current_year = years[0].year();
        for year in years.iter() {
            assert_eq!(current_year, year.year());
            current_year += 1;
        }

        AdventManager { years }
    }

    pub fn solve_day(&self, year: usize, day: usize) {
        // call latest year if year is set to 0
        if year == 0 {
            return self.years.last().unwrap().solve(day);
        }

        let first_year = self.years[0].year();

        assert!(year >= first_year && year < (first_year + self.years.len()),
        "{} has no available solutions. Solutions are available for the years {}-{}. 0 for latest", year, first_year, first_year + self.years.len() - 1);

        self.years[year - first_year].solve(day)
    }
}
