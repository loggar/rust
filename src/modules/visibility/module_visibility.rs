#[allow(dead_code)]
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

#[allow(dead_code)]
fn try_me() -> i32 {
    outermost::middle_function();
    // outermost::middle_secret_function(); // private function
    // outermost::inside::inner_function(); // private module
    // outermost::inside::secret_function(); // private module
    return 0;
}

#[cfg(test)]
mod modules_visibility_test {
    #[test]
    fn some_works() {
        let i = crate::modules::visibility::module_visibility::try_me();
        assert_eq!(0, i);
    }
}

// lib.rs
/*
pub mod modules {
    pub mod visibility {
        pub mod module_visibility;
    }
}
*/
// cargo test --lib modules
// cargo test --lib modules::visibility
// cargo test --lib modules::visibility::module_visibility
