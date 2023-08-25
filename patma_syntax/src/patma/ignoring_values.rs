fn foo(_: i32, y: i32) -> () {
    println!("This code only uses the y parameter: {}", y);
}

#[allow(dead_code)]
pub fn run() {
    foo(3, 4);
}