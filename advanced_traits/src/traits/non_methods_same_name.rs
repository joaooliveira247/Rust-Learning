trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    #[allow(unused)]
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}