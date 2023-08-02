#[allow(dead_code)]
pub fn run() {
    let x =5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y)
}