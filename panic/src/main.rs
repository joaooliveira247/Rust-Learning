use std::fs::File;

#[allow(dead_code)]
fn escopo_1() {
    panic!("Break anything.");
}

#[allow(dead_code)]
fn escopo_2() {
    let v = vec![1, 2, 3];
    v[99];
}

fn escopo_3() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}", error);
        },
    };
}

fn main() {
    escopo_3();
}
