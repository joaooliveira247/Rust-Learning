use std::sync::Mutex;

#[allow(dead_code)]
pub fn run() {
    // Using single thread
    let m = Mutex::new(5);

    println!("m = {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}