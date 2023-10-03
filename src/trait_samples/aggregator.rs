pub trait Summarizable {
    fn summarize(&self) -> String;

    fn summarize_with_default_impl(&self) -> String {
        String::from("(Read more...)")
    }
}
