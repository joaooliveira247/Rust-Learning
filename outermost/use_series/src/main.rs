pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                "teste pub function";
            }
        }
    }
}

use a::series::of;

fn main() {
    println!("{}", of::nested_modules());
}