use std::collections::HashMap;

#[allow(dead_code)]
fn escopo_1() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);
}

#[allow(dead_code)]
fn escopo_2() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];

    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:#?}", scores);
}

#[allow(dead_code)]
fn escopo_3() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // a posse de field_name e field_value foi passas para map 
    map.insert(field_name, field_value);

    println!("{:#?}", map);
}

#[allow(dead_code)]
fn escopo_4() {
    // acess values from a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:#?}", score);
}

#[allow(dead_code)]
fn escopo_5() {
    // iter for a hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

#[allow(dead_code)]
fn escopo_6() {
    // update an hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

}

#[allow(dead_code)]
fn escopo_7() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    
}

fn escopo_8() {
    // atualiza valor com base no antigo
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    escopo_8();
}
