#[allow(dead_code)]
#[allow(unused)]
pub fn run() {
    let _x = 5;
    // y is raise a warning.
    let y = 10;
}

#[allow(dead_code)]
pub fn run_2() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found some string");
    }

    println!("{:?}", s);
}