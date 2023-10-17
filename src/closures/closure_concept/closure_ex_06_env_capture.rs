// lib.rs: pub mod closures { ... }
// $ cargo test --lib closures::closure_ex_06_env_capture

#[cfg(test)]
pub mod closure_ex_06_tests {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn run() -> i32 {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        let mut num_sort_operations = 0; // <-- *
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{:#?}, sorted in {num_sort_operations} operations", list);
        return num_sort_operations;
    }

    #[test]
    fn test_run_returns_number_captured() {
        let n = run();
        assert_eq!(6, n, "closure: captures scope");
    }
}
