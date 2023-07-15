use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;

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

#[allow(dead_code)]
fn escopo_7() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
}

#[allow(dead_code)]
fn escopo_8() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file_result = File::open("hello.txt")?;
        let mut username = String::new();
        username_file_result.read_to_string(&mut username)?;
        Ok(username)
    }
}

#[allow(dead_code)]
fn escopo_9() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
}

fn escopo_10() {
    let home = "127.0.0.1".parse::<IpAddr>().unwrap();
}

fn main() {
    
}
