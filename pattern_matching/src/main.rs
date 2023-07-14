#[allow(dead_code)]
fn escopo_1() {
    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(State),
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum State {
        Florida,
        California,
        Alabama,
        Alaska,
        Oregon,
    }

    #[allow(dead_code)]
    fn to_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Quarter state is: {:?}!", state);
                25
            }
        }
    }
    to_cents(Coin::Quarter(State::Florida));
}

#[allow(dead_code)]
fn escopo_2() {
    fn add_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let cinco = Some(5);
    let seis = add_one(cinco);
    let none_ = add_one(None);

    println!("{:?}\n{:?}\n{:?}", cinco, seis, none_)
}

fn escopo_3() {
    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(State),
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum State {
        Florida,
        California,
        Alabama,
        Alaska,
        Oregon,
    }

    #[allow(dead_code)]
    fn to_state(coin: Coin) {
        let mut count_ = 0;

        match coin {
            Coin::Quarter(state) => println!("Qaurter states is {:?}", state),
            _ => count_ += 1,
        }
    }

    fn to_state_2(coin: Coin) {
        let mut count_ = 0;
        if let Coin::Quarter(state) = coin {
            println!("Quarter state is: {:?}!", state);
        } else {
            count_ += 1;
        }
    }
}

fn main() {
    escopo_2();
}
