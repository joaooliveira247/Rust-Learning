#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
pub fn run() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data do destructure.");
        },
        Message::Move { x, y } => {
            println!("Move in the direction {x} and in the y direction {y}");
        },
        Message::Write(text) => {
            println!("Text message: {text}");
        },
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, 'n' blue {b}");
        }
    }
}