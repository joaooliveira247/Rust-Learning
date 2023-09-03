#[allow(dead_code)]
pub fn run() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

#[allow(dead_code)]
#[allow(unused)]
pub fn run_2() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_lon_type(f: Thunk) {}

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }
}
