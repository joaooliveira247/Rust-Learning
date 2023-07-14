fn main() {
    // loop_();
    // while_();
    // while_array();
    // for_array();
    for_range();
}

#[allow(dead_code)]
fn loop_() {
    loop {
        println!("Again");
    }
}

#[allow(dead_code)]
fn while_() {
    let mut num = 3;

    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }

    println!("LIFTOFF!!!");
}

#[allow(dead_code)]
fn while_array() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < arr.len() {
        println!("The value is: {}", arr[index]);
        index += 1;
    }
}

#[allow(dead_code)]
fn for_array() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    for i in arr.iter() {
        println!("The value is :{}", i);
    }
}

fn for_range() {
    for i in (1..4).rev() {
        println!("{}!", i);
    }
    println!("LIFTOFF!!!");
}