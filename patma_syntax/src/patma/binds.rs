enum Message {
    Hello { id: i32 },
}

#[allow(dead_code)]
pub fn run() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_var @ 3..=7 } => println!("Found an id id range: {}", id_var),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
