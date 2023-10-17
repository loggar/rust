// lib.rs: pub mod closures { ... }
// $ cargo test --lib closures::closure_ex_05_env_capture

#[cfg(test)]
pub mod closure_ex_05_tests {
    fn closure_captures_outside_scope() -> bool {
        let x = 4;

        let equal_to_x = |z| z == x;

        let y = 4;

        return equal_to_x(y);
    }

    #[test]
    fn test_closure_captures_outside_scope() {
        let b = closure_captures_outside_scope();
        assert!(b);
    }
}
