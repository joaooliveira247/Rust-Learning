#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect_1 = Rectangle {
        length: 50,
        width: 30,
    };

    let rect_4 = Rectangle::square(30);

    println!("rect_4 is: {:#?}", rect_4);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_1.area()
    );
    let rect_2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect_3 = Rectangle {
        length: 45,
        width: 60,
    };
    println!("Can rect1 hold rect2? {}", rect_1.can_hold(&rect_2));
    println!("Can rect1 hold rect3? {}", rect_1.can_hold(&rect_3));
}
