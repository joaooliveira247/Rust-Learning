enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

#[allow(dead_code)]
#[allow(unused)]
fn run() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
