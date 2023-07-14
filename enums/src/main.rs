#[derive(Debug)]
enum Message {
    Exit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn evoke(&self) {}
}

#[allow(dead_code)]
fn escopo_1() {
    #[derive(Debug)]
    enum IpVersion {
        V6,
        V4,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct AddressIp {
        version: IpVersion,
        ip: String,
    }

    let local = AddressIp {
        version: IpVersion::V4,
        ip: String::from("127.0.0.1"),
    };

    let loopback = AddressIp {
        version: IpVersion::V6,
        ip: String::from("::1"),
    };

    println!("{:#?}\n{:#?}", local, loopback);
}

#[allow(dead_code)]
fn escopo_2() {
    #[derive(Debug)]
    enum IpAddress {
        V6(String),
        V4(String),
    }

    let local = IpAddress::V4(String::from("127.0.0.1"));

    let loopback = IpAddress::V6(String::from("::1"));

    println!("{:#?}\n{:#?}", local, loopback);
}

#[allow(dead_code)]
fn escopo_3() {
    #[derive(Debug)]
    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let local = IpAddress::V4(127, 0, 0, 1);

    let loopback = IpAddress::V6(String::from("::1"));

    println!("{:#?}{:#?}", local, loopback);
}

fn escopo_4() {
    let m = Message::Write(String::from("Hello"));
    m.evoke();
}
fn main() {
    escopo_3();
}
