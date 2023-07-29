#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref_1 = Some(ShirtColor::Red);
    let giveaway_1 = store.giveaway(user_pref_1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_1, giveaway_1
    );
    let user_pref_2 = None;
    let giveaway_2 = store.giveaway(user_pref_2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_2, giveaway_2
    );

}