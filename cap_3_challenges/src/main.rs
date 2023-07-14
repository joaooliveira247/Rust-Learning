fn main() {
    chrismass_letter()
}

#[allow(dead_code)]
fn fahrenheit_to_celsis(f: f32) -> f32 {
    // c = 5(f - 32)/9
    return 5.0 * ((f - 32.0) / 9.0);
}

#[allow(dead_code)]
fn n_fibonacci(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();

    for i in 0..n as usize {
        if i < 2 {
            vec.push(1);
        } else {
            vec.push(vec[i - 2] + vec[i - 1]);
        }
    }
    return vec;
}

fn chrismass_letter() {
    let verse_arr: [(&str, &str); 12] = [
        ("first", "A partridge in a pear tree"),
        ("second", "two turtle doves,"),
        ("third", "Three French hens,"),
        ("fouth", "Four calling birds,"),
        ("fifth", "Five golden rings,"),
        ("sixth", "Six geese a-laying,"),
        ("seventh", "Seven swans a-swimming,"),
        ("eighth", "Eight maids a-milking,"),
        ("nineth", "Nine ladies dancing,"),
        ("tenth", "Ten lords a-leaping,"),
        ("eleventh", "Eleven pipers piping,"),
        ("twelfth", "Twelve drummers drumming,"),
    ];

    let mut teste_2: Vec<&str> = Vec::new();

    for (i, verse) in verse_arr.iter().enumerate() {
        teste_2.insert(0, verse.1);
        println!(
            "On the {} day of Christmas,\nmy true love sent to me",
            verse.0
        );
        println!("{}", teste_2.join("\n"));
        if i > 0 {
            println!("And {}", verse_arr[0].1.replace("A", "a"));
        } else {
            teste_2.pop();
        }
        println!();
    }
}
