pub trait Resume {
    fn resume(&self) -> String {
        String::from("(Leia mais...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Resume for NewsArticle {}