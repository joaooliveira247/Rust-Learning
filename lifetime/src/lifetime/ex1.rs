#[allow(dead_code)]
#[allow(unused)]
pub fn dangling() {
    // let r;
    {
        let x = 5;
        // r = &x;
    }
    println!("r: ", );
}

pub fn its_work() {
    let x = 5;

    let r = &x;

    println!("r: {}", r);
}
