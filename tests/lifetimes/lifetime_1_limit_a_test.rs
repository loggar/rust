/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

error[E0106]: missing lifetime specifier
 --> tests/lifetimes/lifetime_limit_a_test.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn limit_lifetime_a() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
    assert_eq!(string1, result, "longest function with named lifetime parameter");
}

#[test]
fn limit_lifetime_a_correct_lifetime_delivered() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);
        assert_eq!(string1, result, "longest function with named lifetime parameter");
    }
}

/*
#[test]
fn limit_lifetime_a_incorrect_lifetime_delivered() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // `string2` does not live long enough
    }

    println!("The longest string is {}", result);
    assert_eq!(string1, result, "longest function with named lifetime parameter");
}
 */

fn longest_no_need_to_specify_y_lifetime<'a>(x: &'a str, y: &str) -> &'a str {
    println!(
        "don't specify y's lifetime which is not relevant to the returned lifetime {}",
        y
    );
    x
}

#[test]
fn limit_lifetime_a_only_specify_relevant_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_no_need_to_specify_y_lifetime(string1.as_str(), string2);

    println!("The longest string is {}", result);
    assert_eq!(string1, result, "longest function with named lifetime parameter");
}

/*
fn longest_returned_lifetime_must_be_related<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

error[E0515]: cannot return reference to local variable `result`
  --> tests/lifetimes/lifetime_limit_a_test.rs:89:5
   |
89 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
*/
