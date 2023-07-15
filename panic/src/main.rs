use std::fs::File;
use std::io::ErrorKind;

#[allow(dead_code)]
fn escopo_1() {
    panic!("Break anything.");
}

#[allow(dead_code)]
fn escopo_2() {
    let v = vec![1, 2, 3];
    v[99];
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn escopo_3() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}", error);
        }
    };
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn escopo_4() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Rust try create a file 'n' something happen in {:?}", e)
            }
        },
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo {:?}", error)
        }
    };
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn escopo_5() {
    let f = File::open("hello.txt").unwrap();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn escopo_6() {
    let f = File::open("hello.txt").expect("Error to open hello.txt");
}

fn escopo_7() {
    
}

fn main() {
    escopo_6();
}
