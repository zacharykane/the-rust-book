pub trait Summary {
    // define a default implementation when a type doesn't declare its own
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub content: String,
    pub headline: String,
    pub author: String,
    pub location: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Toot {
    pub content: String,
    pub username: String,
    pub is_reply: bool,
    pub is_retoot: bool,
}
impl Summary for Toot {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct BlogPost {
    pub content: String,
    pub author: String,
    pub web_ring: String,
}
impl Summary for BlogPost {}

// define a function that accepts: a reference, to any type that implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Hmm! {}", item.summarize());
}
