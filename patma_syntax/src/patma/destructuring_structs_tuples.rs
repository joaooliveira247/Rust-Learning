struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
pub fn run() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("Tuple values : ({feet}, {inches})\nPoint values: ({x}, {y})");
}
