use std::fmt::{Display, Formatter, Result};

pub struct Report<T: Display, U: Display> {
    pub exercise: u8,
    pub first: T,
    pub second: U,
}

impl<T: Display, U: Display> Display for Report<T, U> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Result ex. {}: first: {} second: {}",
            self.exercise, self.first, self.second
        )
    }
}
