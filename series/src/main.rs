#[allow(dead_code)]
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() -> String{
                String::from("this is a examples of 'use' cases")
            }
        }
    }
}

use a::series::of;

enum TraficLight {
    Red,
    Yellow,
    Green
}

// use TraficLight::{Red, Yellow};

use TraficLight::*;


fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
