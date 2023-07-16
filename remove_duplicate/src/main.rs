mod remove_duplicate;

use  remove_duplicate::ex3::largest;

fn main() {
    // remove_duplicate::ex1::largest();
    // remove_duplicate::ex2::largest_two_times();
    let nums = vec![34, 50, 25, 100, 65];

    let res = largest(&nums);

    println!("The largest number is {}", res);

    let nums = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let res = largest(&nums);

    println!("The largest number is {}", res);
}
