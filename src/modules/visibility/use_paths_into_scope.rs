mod front_of_house {
    #[allow(dead_code)]
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::modules::visibility::use_paths_into_scope::front_of_house::hosting;

#[allow(dead_code)]
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// Bringing the function level into scope with use, which is unidiomatic.
// Bringing module level into scope in an idiomatic way.
use std::collections::HashMap;

#[allow(dead_code)]
pub fn bring_mod_level_via_use() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// There’s no strong reason behind this idiom: it’s just the convention that has
// emerged, and folks have gotten used to reading and writing Rust code this way.

// Rust doesn't allow bringing same name into scope,
// how to bring two Result types into scope:
use std::fmt;
use std::io;

#[allow(dead_code, unused_variables)]
fn function1(p: fmt::Result) {}
#[allow(dead_code, unused_variables)]
fn function2(p: io::Result<()>) {}
