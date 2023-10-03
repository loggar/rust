pub trait Summarizable {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[test]
fn trait_simple_test() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    assert_eq!(
        "horse_ebooks: of course, as you probably already know, people",
        tweet.summarize(),
        "trait Summarizable.summarize"
    );
}
