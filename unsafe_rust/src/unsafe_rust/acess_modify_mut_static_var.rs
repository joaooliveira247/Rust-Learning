#[allow(unused)]
static HELLO_WORLD: &str = "Hello, world!";

#[allow(dead_code)]
pub fn run() {
    println!("name is: {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[allow(dead_code)]
pub fn run_2() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}