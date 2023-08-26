#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x , y , z }
    }
}

#[allow(dead_code)]
pub fn run() {
    let origin = Point::new(0, 0, 0);

    match origin {
        Point {x, ..} => println!("x is {}", x),
    }
}

#[allow(dead_code)]
pub fn run_2() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
