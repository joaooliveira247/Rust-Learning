use std::fmt::Display;

pub struct Double<T> {
    x: T,
    y: T
}

impl<T> Double<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T: Display + PartialOrd> Double<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The great number is {}", self.x);
            return;
        }
        println!("The great number is {}", self.y);
    }
}