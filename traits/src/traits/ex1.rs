pub trait Resume {
    fn resume(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Resume for NewsArticle {
    fn resume(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Resume for Tweet {
    fn resume(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}