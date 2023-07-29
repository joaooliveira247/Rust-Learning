#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height}
    }
}

pub fn run() {
    let mut list = [
        Rectangle::new(3, 5),
        Rectangle::new(7, 12),
        Rectangle::new(10, 1),
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list)
}