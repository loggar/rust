mod sub_mod {
    #[allow(dead_code)]
    pub fn some() -> i8 {
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn some_works() {
        let i = crate::lib_sub_mod::sub_mod::some();
        assert_eq!(0, i);
    }
}

// lib.rs: pub mod lib_sub_mod
// $ cargo test --lib lib_sub_mod
