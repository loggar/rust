#[cfg(test)]
mod lifetimes {
    pub mod lifetime_1_limit_a_test;
    pub mod lifetime_2_elision_test;
    pub mod lifetime_3_in_struct_test;
}

// $ cargo test --test _t_lifetimes_tests
