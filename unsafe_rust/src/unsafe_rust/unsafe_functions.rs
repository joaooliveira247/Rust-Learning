unsafe fn dangerous() {}

#[allow(dead_code)]
pub fn run() {
    unsafe {
        dangerous();
    }
}