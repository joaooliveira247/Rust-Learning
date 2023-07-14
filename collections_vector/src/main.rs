#[allow(dead_code)]
fn escopo_1() {
    let mut new_vec: Vec<i32> = Vec::new();
    new_vec.push(10);
    new_vec.push(20);
    new_vec.push(30);
    new_vec.push(99);

    let value = new_vec.get(1);

    println!("{:#?}", value);

    if value != None {
        println!("{:#?}", value);
    } else {
        println!("value out of index.");
    }
}

fn escopo_2() {
    // armazendo varios tipos em um vec
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12), 
        ];
    
    println!("{:#?}", row);
}

fn main() {
    escopo_2();
}