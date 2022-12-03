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
        assert!(day > 0 && day < 26, "day must be 1-25");

        if day >= self.days.len() {
            println!("unimplemented");
        }

        self.days[day]();
    }

    fn solve_latest(&self) {
        if self.days.len() == 0 {
            println!("unimplemented");
            return;
        }

        
        self.days.last().unwrap()();
    }
}
