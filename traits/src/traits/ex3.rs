pub trait Resume {
    fn author_resume(&self) -> String;

    fn resume(&self) -> String {
        format!("(Leia mais de {} ...)", self.author_resume())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Resume for Tweet {
    fn author_resume(&self) -> String {
        format!("@{}", self.username)
    }
}