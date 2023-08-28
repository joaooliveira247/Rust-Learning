extern "C" {
    #[allow(dead_code)]
    fn abs(input: i32) -> i32;
}

#[allow(dead_code)]
pub fn run() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
