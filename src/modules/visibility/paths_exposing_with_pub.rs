mod front_of_house {
    pub mod hosting {
        #[allow(dead_code)]
        pub fn add_to_waitlist() {}
    }
}

#[allow(dead_code)]
pub fn eat_at_restaurant() {
    // Absolute path
    crate::modules::visibility::paths_exposing_with_pub::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
