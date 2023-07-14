mod interprise;
mod mean;
mod pig_latin;

use interprise::Interprise;
use mean::Values;
use pig_latin::pig_latin;
use std::env::args;
use std::io::stdin;

fn main() {
    let x: Vec<String> = args().collect();
    match x[1].to_lowercase().as_str() {
        "piglatin" => println!("{}", pig_latin(x[2].to_string())),
        "mean" => {
            let y = Values { values: x };
            println!("{:?}", y.mean());
        }
        "interprise" => {
            let mut users = Interprise::new();
            loop {
                let mut user_input = String::new();
                stdin()
                    .read_line(&mut user_input)
                    .expect("command not found");
                let command: Vec<String> = user_input
                    .split(" ")
                    .map(|x| x.trim().to_string())
                    .collect();
                match command[0].to_lowercase().as_str() {
                    "add" => {
                        users.add(
                            command[1].to_string(), 
                            command[2].to_string()
                        );
                    }
                    "delete" => {
                        users.delete(
                            command[1].to_string(),
                             command[2].to_string());
                    }
                    "read" => println!("{:?}", users.read()),
                    "exit" => break,
                    _ => println!("command not found"),
                }
            }
        }
        _ => println!("Option not found"),
    }
}
