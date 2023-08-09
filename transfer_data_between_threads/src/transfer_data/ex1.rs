use std::sync::mpsc;
use std::thread;

#[allow(unused)]
#[allow(dead_code)]
pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // send rubber duck inside the river.
        tx.send(val).unwrap();
        // ownership o 'val' moved to main thread.
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received)
}