fn main() {
    // divisible_by(7);
    var_bool();
}

fn divisible_by(num: i32) {
    if num % 5 == 0 {
        println!("{} is divisible by 5", num);
    } else if num % 3 == 0 {
        println!("{} is divisible by 3", num);
    } else if num % 2 == 0 {
        println!("{} is divisible by 2", num);
    } else {
        println!("{} isn't divible by [2, 3, 5]", num);
    }
}

fn var_bool() {
    let condition = true;
    let num = if condition {
        5
    } else {
        6
    };

    println!("Num value is: {}", num)
}
