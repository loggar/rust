#[test]
fn assign_from_if_statement_test() {
    let is_leap_year = 2024 % 4 == 0;
    let days_in_year = if is_leap_year { 366 } else { 365 };
    assert_eq!(true, is_leap_year, "Unexpected");
    assert_eq!(366, days_in_year, "Unexpected");
}
