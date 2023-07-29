use std::{thread, time::Duration};

#[allow(dead_code)]
pub fn run() {
    let expensive_clousure = |num: u32| -> u32 {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("{}", expensive_clousure(5));
}