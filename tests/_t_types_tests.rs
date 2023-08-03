mod types {
    pub mod arrays {
        pub mod array_1_test;
    }
    pub mod scalar_types {
        pub mod bool_1_test;
        pub mod char_1_test;
        pub mod numbers_1_simple_test;
    }

    pub mod slices {
        pub mod slice_1_string_test;
        pub mod slice_2_int_test;
    }

    pub mod type_names {
        pub mod crate_type_name_usage_test;
        pub mod lib_get_type_of_usage_test;
    }
    pub mod tuples {
        pub mod tuple_1_simple_test;
        pub mod tuple_2_type_test;
    }
    pub mod structs {
        pub mod struct_1_simple_test;
        pub mod struct_2_builder_test;
        pub mod struct_3_tuple_struct_test;
        pub mod struct_4_unit_like_struct_without_field_test;
        pub mod struct_5_debug_test;
        pub mod struct_6_method_test;
    }
}

// $ cargo test --test _t_types_tests
