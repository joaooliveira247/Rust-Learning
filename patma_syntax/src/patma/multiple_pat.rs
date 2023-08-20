#[allow(dead_code)]
pub fn run() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("any"),
    }
}

