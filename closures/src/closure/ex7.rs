use std::thread;

#[allow(dead_code)]
pub fn run() {
    let list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
