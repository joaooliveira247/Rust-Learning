fn main() {
    escopo_4_2  ();
}

#[allow(dead_code)]
fn escopo_4() {
    let s1 = String::from("texto");
    let size = calcula_tamanho(&s1);
    println!("O tamanho de '{s1}' é {size}");
}

#[allow(dead_code)]
fn calcula_tamanho(text: &String) -> usize {
    text.len()
}

#[allow(dead_code)]
fn escopo_4_1() {
    let mut s = String::from("Text");
    modifica(&mut s);
    println!("{s}");
}

#[allow(dead_code)]
fn modifica(text: &mut String) {
    text.push_str(" long");
}

fn escopo_4_2() {
    multiple_references();
}

fn multiple_references() {
    let mut s = String::from("Hello, ");

    {
        let r1 = &mut s;
        r1.push_str("Multiple ");
    }
    let s2 = &mut s;
    s2.push_str("references.");

    println!("{s2}");
}
