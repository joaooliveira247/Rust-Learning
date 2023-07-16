mod generics;

use generics::ex1::{largest_char, largest_i32};
// use generics::ex2::largest;
// use generics::ex3::Point;
// use generics::ex4::Point;
// use  generics::ex5::Point;
use generics::ex6::Point;

#[allow(dead_code)]
fn escopo_1() {
    let list_nums = vec![34, 50, 25, 100, 65];

    let res_i32 = largest_i32(&list_nums);
    println!("The largest number is {}", res_i32);

    let list_chars = vec!['y', 'm', 'a', 'q'];

    let res_char = largest_char(&list_chars);

    println!("The largest char is {}", res_char);
}
#[allow(dead_code)]
#[allow(unused_variables)]
fn escopo_2() {
    let list_nums = vec![34, 50, 25, 100, 65];

    // let res_i32 = largest(&list_nums);
    // println!("The largest number is {}", res_i32);

    let list_chars = vec!['y', 'm', 'a', 'q'];

    // let res_char = largest(&list_chars);

    // println!("The largest char is {}", res_char);
}

#[allow(dead_code)]
fn escopo_3() {
    let int = Point {x: 5, y: 10};

    let float = Point {x: 2.0, y: 8.0};

    println!("{:?}", int);

    println!("{:?}", float);
}

#[allow(dead_code)]
fn escopo_4() {
    let integers = Point {x: 5, y: 2};

    let floats = Point {x: 4.0, y: -3.0};

    let int_n_float = Point {x: -10, y: 18};

    println!("{:?}\n{:?}\n{:?}", integers, floats, int_n_float);
}

#[allow(dead_code)]
fn escopo_5() {
    // let p = Point::new(12, -11);

    // println!("p.x = {}", p.x());
}

fn escopo_6() {
    let p: Point<f32> = Point {x: 12.0, y:5.0};

    println!("{:?}", p.origin_distance())
}

fn main() {
    escopo_6();
}
