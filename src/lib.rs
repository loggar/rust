pub mod lib_simple; // ./lib_simple.rs
pub mod lib_types; // export lib: use lib_root::lib_types::get_type_of;

pub mod modules {
    pub mod visibility {
        pub mod make_enum_public;
        pub mod make_struct_public;
        pub mod module_visibility; // ./modules/visibility/module_visibility.rs
        pub mod paths_exposing_with_pub;
        pub mod paths_relative_with_super;
        pub mod use_as_new_name;
        pub mod use_paths_into_scope;
    }
}

#[cfg(test)]
mod lib_tests {
    #[test]
    fn it_works() {}
}

// $ cargo test --lib
