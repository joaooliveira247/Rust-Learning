mod traits;

use traits::ex1::Tweet;
use traits::ex2::NewsArticle;

// use crate::traits::ex1::Resume;
use crate::traits::ex2::Resume;

fn escopo_1() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("pf course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.resume());
}

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

    println!("{}", article.resume());
}

fn main() {
    escopo_2();
}
