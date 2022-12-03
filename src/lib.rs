pub mod day1;
pub mod year2022;

pub trait AdventYear {
    fn year(&self) -> usize;
    fn solve(&self, day: usize);
    fn solve_latest(&self);
}

pub struct Year {
    year: usize,
    days: Vec<Box<dyn Fn()>>,
}

impl AdventYear for Year {
    fn year(&self) -> usize {
        self.year
    }

    fn solve(&self, day: usize) {
        if day == 0 {
            return self.solve_latest();
        }

        assert!(day > 0 && day < 26, "day must be 1-25");

        if day >= self.days.len() {
            println!("unimplemented");
        }

        self.days[day]()
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
    first_year: usize,
    years: Vec<Box<dyn AdventYear>>,
}

impl AdventManager {
    /// Constructs a new AdventManager
    ///
    /// # Invariants
    /// `first_year` must be the first year contained in `years`.
    /// `years` must be a sequential list of AdventYear trait objects
    pub fn new(first_year: usize, years: Vec<Box<dyn AdventYear>>) -> AdventManager {
        // check function invariants
        let mut current_year = first_year;
        for year in years.iter() {
            assert_eq!(current_year, year.year());
            current_year += 1;
        }


        AdventManager {
            first_year,
            years,
        }
    }


    pub fn solve_day(&self, year: usize, day: usize) {
        assert!(year >= self.first_year && year < (self.first_year + self.years.len()),
        "{} has no available solutions. Solutions are available for the years {}-{}", year, self.first_year, self.first_year + self.years.len() - 1);

        self.years[year - self.first_year].solve(day)
    }
}

pub fn init() -> AdventManager {
    let mut years = vec![];

    years.push(year2022::init());

    AdventManager::new(2022, years)
}
