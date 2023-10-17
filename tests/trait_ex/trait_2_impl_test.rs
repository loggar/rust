mod default_impl_tests {
    use lib_root::traits::aggregator::Summarizable;

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
    fn trait_impl_test() {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        };

        assert_eq!(
            "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)",
            article.summarize(),
            "trait Summarizable.summarize"
        );

        assert_eq!(
            "(Read more...)",
            article.summarize_with_default_impl(),
            "trait Summarizable.summarize"
        );
    }
}
