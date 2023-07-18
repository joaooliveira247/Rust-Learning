mod traits;

use std::vec;

// use traits::ex1::Tweet;
use traits::ex2::NewsArticle;
use traits::ex3::Tweet;
use traits::ex4::largest;
use traits::ex5::Double;

use crate::traits::ex3::Resume;

// use crate::traits::ex1::Resume;
// use crate::traits::ex2::Resume;

#[allow(dead_code)]
#[allow(unused_variables)]
fn escopo_1() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("pf course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.resume());
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn escopo_2() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // println!("{}", article.resume());
}

#[allow(dead_code)]
fn escopo_3() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("pf course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.resume());
}

#[allow(dead_code)]
fn escopo_4() {
    let list_int = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&list_int));

    let list_char = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is '{}'", largest(&list_char));
}

fn escopo_5() {
    let nums = Double::new(999, 2);
    nums.cmp_display();
}

fn main() {
    escopo_5();
}
