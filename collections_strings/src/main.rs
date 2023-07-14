use std::fmt::format;

#[allow(dead_code)]
fn escopo_1() {
    let mut s = String::new(); // Empty string

    let data = "initial contents"; // &str

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

}

#[allow(dead_code)]
fn escopo_2() {
    // update an string.
    let mut s = String::from("foo");
    // o metodo push_str leva um &str por precisar de uma referencia e não ser owned
    s.push_str("bar");

    let mut s = String::from("lo");
    // o método push pede um char como param
    s.push('l');
}

#[allow(dead_code)]
fn escopo_3() {
    // concatenando string

    let s1 = String::from("Hello ");
    let s2 = String::from("World");

    let s3 = s1 + &s2; // a posse de s1 foi dada para s3

    println!("{s3}");
}

#[allow(dead_code)]
fn escopo_4() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{s}");
}

#[allow(dead_code)]
fn escopo_5() {
    // string slicing
    // let hello = "Здравствуйте";

    // let s = &hello[..4];

    // for c in "नमस्ते".chars() {
    //     println!("{c}");
    // }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // println!("{s}");
}

fn main() {
    escopo_5(); 
}
