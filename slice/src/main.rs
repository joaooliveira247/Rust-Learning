fn main() {
    let s = String::from("Long Text");
    let word = first_word(&s[..]);
    println!("{word}");
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let slice = &a[1..3];
    println!("{:?}", slice)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
