#[allow(dead_code)]
pub fn largest() {
    let nums = vec![34, 50, 25, 100, 65];

    let mut largest = nums[0];

    for num in nums {
        if num > largest {
            largest = num;
        }
    }

    println!("The largest number is {}", largest)
}