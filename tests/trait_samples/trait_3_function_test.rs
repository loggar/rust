#[cfg(test)]
mod trait_in_function_tests {
    use lib_root::trait_samples::aggregator::Summarizable;

    pub fn notify(item: &impl Summarizable) -> String {
        return format!("Breaking news! {}", item.summarize());
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summarizable for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
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

    fn returns_summarizable() -> impl Summarizable {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    #[test]
    fn trait_as_function_parameter_test() {
        let article = NewsArticle {
            headline: String::from("A"),
            location: String::from("B"),
            author: String::from("C"),
            content: String::from("D"),
        };

        assert_eq!(
            "Breaking news! A, by C (B)",
            notify(&article),
            "trait as a function parameter"
        );
    }

    #[test]
    fn trait_as_return_type_test() {
        let article = returns_summarizable();

        assert_eq!(
            "Breaking news! horse_ebooks: of course, as you probably already know, people",
            notify(&article),
            "trait as a return type"
        );
    }
}
