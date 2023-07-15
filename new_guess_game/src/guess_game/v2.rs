extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[allow(dead_code)]
fn guess_game_1() {
    println!("Guess a number!");

    let secret_num: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please, input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess <1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("You win ;)");
                break;
            }
        }
    }
}