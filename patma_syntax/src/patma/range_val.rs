#[allow(dead_code)]
pub fn run_numeric() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("somethinf else"),
    }
}

#[allow(dead_code)]
pub fn run_char() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}