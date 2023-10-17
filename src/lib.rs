pub mod closures {
    mod closure_concept {
        mod closure_ex_05_env_capture;
        mod closure_ex_06_env_capture;
    }
}

pub mod lib_simple; // ./lib_simple.rs
pub mod lib_types; // export lib: use lib_root::lib_types::get_type_of;

pub mod modules {
    mod visibility {
        mod make_enum_public;
        mod make_struct_public;
        mod module_visibility; // ./modules/visibility/module_visibility.rs
        mod paths_exposing_with_pub;
        mod paths_relative_with_super;
        mod use_as_new_name;
        mod use_paths_into_scope;
        mod use_with_pub_re_export;
    }

    mod externals {
        mod cargo_dependency;
        mod nested_paths_to_clean_up_use_list;
    }
}

pub mod traits {
    pub mod aggregator;
}

#[cfg(test)]
mod lib_tests {
    #[test]
    fn it_works() {}
}

// $ cargo test --lib
