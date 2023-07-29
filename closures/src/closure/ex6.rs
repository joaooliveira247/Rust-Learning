#[allow(dead_code)]
#[allow(unused)]
pub fn run() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = || list.push(7);

    borrow_mutably();

    println!("After calling closure: {:?}", list);
}