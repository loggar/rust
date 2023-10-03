#[cfg(test)]
mod bound_simple_tests {
    use lib_root::trait_samples::aggregator::Summarizable;

    pub fn notify<T: Summarizable>(item: T) -> String {
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

    #[test]
    fn trait_bound_test() {
        let article = NewsArticle {
            headline: String::from("A"),
            location: String::from("B"),
            author: String::from("C"),
            content: String::from("D"),
        };

        assert_eq!("Breaking news! A, by C (B)", notify(article), "trait bound - notify");
    }
}
