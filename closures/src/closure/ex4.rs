#[allow(dead_code)]
#[allow(unused)]
pub fn run() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}