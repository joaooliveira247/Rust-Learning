struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    escopo_5_2();
}

#[allow(dead_code)]
fn escopo_5_1() {
    let mut user_1 = User {
        email: String::from("alguem@ecemplo.com"),
        username: String::from("alguem"),
        active: true,
        sign_in_count: 1,
    };
    user_1.email = String::from("Another_One");
    // struct update syntax

    #[warn(unused_variables)]
    let user_2 = User {
        email: String::from("update_email@gmail.com"),
        username: String::from("update_user"),
        active: user_1.active,
        sign_in_count: user_1.sign_in_count,
    };

    // outra maneira de fazer update

    #[warn(unused_variables)]
    let user_2 = User {
        email: String::from("nem_email@gmail.com"),
        username: String::from("username"),
        ..user_1
    };

}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[allow(dead_code)]
fn struct_tuples() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);

    let origin = Point(0, 0, 0);
}

fn escopo_5_2() {
    // let lenght_1 = 50;
    // let width_1 = 30;
    // let rect_1: (u32, u32) = (50, 30);
    let rect_1 = RectDim {
        lenght: 50,
        width: 30
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect_1)
    );
    println!("Rectangle is {:#?}", rect_1);
}

#[allow(dead_code)]
fn area(lenght: u32, width: u32) -> u32 {
    lenght * width
}

#[allow(dead_code)]
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct RectDim {
    lenght: u32,
    width: u32
}

fn struct_area(dimensions: &RectDim) -> u32 {
    dimensions.lenght * dimensions.width
}