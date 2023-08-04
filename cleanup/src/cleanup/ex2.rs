use crate::CustomSmartPointer;

pub fn run() {
    let c = CustomSmartPointer {
        data: String::from("some data")
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}