fn main() {
    let x = sum(5, 17);
    // let x = five();
    println!("The value of x is: {}", x)
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn five() -> i32 {
    5
}