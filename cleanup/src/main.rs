mod cleanup;

use cleanup::ex2;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[allow(dead_code)]
#[allow(unused)]
fn main() {
    ex2::run();
}
