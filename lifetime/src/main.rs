mod lifetime;

use lifetime::{ex1, ex2, ex3};

#[allow(dead_code)]
fn escopo_1() {
    ex1::its_work();
}

#[allow(dead_code)]
fn escopo_2() {
    let string_1 = String::from("abcd");
    let string_2 = "xyz";

    let result = ex2::largest(string_1.as_str(), string_2);
    println!("The largest string is {}", result);
}

#[allow(dead_code)]
fn escopo_3() {
    let string_1 = String::from("The largest string is largest");
    {
        let string_2 = String::from("xyz");
        let result = ex2::largest(string_1.as_str(), string_2.as_str());
        println!("The largest string is {}", result);
    }
}

#[allow(dead_code)]
fn escopo_4() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_setence = novel.split('.').next().expect("Could not find a '.'");
    let i = ex3::ImportantExcerpt {
        part: first_setence,
    };
    println!("{:?}", i);
}

fn main() {
    escopo_4();
}
