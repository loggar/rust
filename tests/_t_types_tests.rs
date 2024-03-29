mod types {
    pub mod arrays {
        pub mod array_1_test;
    }
    pub mod enums {
        pub mod enum_1_variants_test;
        pub mod enum_2_variants_data_test;
        pub mod enum_3_variants_struct_data_test;
        mod enum_match_1_simple_test;
        mod enum_match_2_discern_test;
        mod enum_match_3_self_test;
    }

    pub mod generics {
        pub mod generics_function_arg_test;
        pub mod generics_struct_method_test;
        pub mod generics_struct_simple_test;
    }

    pub mod options {
        pub mod options_1_value_match_test;
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
        pub mod struct_7_instance_ownership_test;
        pub mod struct_8_tuple_struct_test;
    }
}

// $ cargo test --test _t_types_tests
