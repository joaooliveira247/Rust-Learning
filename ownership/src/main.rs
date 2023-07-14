fn main() {
    escopo_4_4();
}

#[allow(dead_code)]
fn owner_string() {
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{s}");
}

#[allow(dead_code)]
fn owner_clone() {
    let s1 = String::from("Texto");
    let s2 = s1.clone();
    println!("s1: {s1} | s2: {s2}");
}

#[allow(dead_code)]
fn escopo_4() {
    let s = String::from("text");
    toma_posse(s); //A posse de s é movida para text, q é arg de toma_posse()

    // println!("{s}"); caso tente printar s ele vai dar o erro de borrow move

    let x = 5;
    faz_copia(x);
    println!("{x}"); // nesse momento como foi copiado ele sera exibido sem errp
}

#[allow(dead_code)]
fn toma_posse(text: String) {
    println!("{text}"); // tipo composto, logo ele ira tomar posse.
}

#[allow(dead_code)]
fn faz_copia(i: i32) {
    println!("{i}"); // tipo primitivo, logo ele tem a trait copy.
}

#[allow(dead_code)]
fn escopo_4_3() {
    let s1 = entrega_valor();
    // a posse de hello é movida de one_string para s1 quando termina o escopo
    // de entrega valor.

    println!("{s1}");
    
    let s2 = String::from("Text");

    let s3 = pega_e_entrega_valor(s2);
    println!("{s3}");
    // s2 tem a posse de "texte", quando entra em pega_e_entrega_value a posse
    // é de s, quando sai a posse de "Text" é de s3
}

#[allow(dead_code)]
fn entrega_valor() -> String {
    let one_string = String::from("Hello");
    one_string
}

#[allow(dead_code)]
fn pega_e_entrega_valor(s: String) -> String {
    s
}

fn escopo_4_4() {
    // exemplo usado para dizer q é mt cerimonia n usar referencia para usar um
    // mesmo valor.
    let s1 = String::from("Texto");
    let (s2, tamanho) = calcula_tamanho(s1);
    println!("O tamanho de '{s2}' é {tamanho}");
}

fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len();
    (s, tamanho)
}