use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run() {
    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w = {}", w);
}
