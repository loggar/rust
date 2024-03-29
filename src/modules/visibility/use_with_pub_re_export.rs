mod front_of_house {
    pub mod hosting {
        #[allow(dead_code)]
        pub fn add_to_waitlist() {}
    }
}

pub use crate::modules::visibility::use_with_pub_re_export::front_of_house::hosting;

#[allow(dead_code)]
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// Before this change, external code would have to call the add_to_waitlist function
// by using the path restaurant::front_of_house::hosting::add_to_waitlist().
// Now that this pub use has re-exported the hosting module from the root module,
// external code can now use the path restaurant::hosting::add_to_waitlist() instead.

// Re-exporting is useful when the internal structure of your code is different
// from how programmers calling your code would think about the domain. For example,
// in this restaurant metaphor, the people running the restaurant think about
// “front of house” and “back of house.” But customers visiting a restaurant probably
// won’t think about the parts of the restaurant in those terms. With pub use, we
// can write our code with one structure but expose a different structure. Doing
// so makes our library well organized for programmers working on the library and
// programmers calling the library.
