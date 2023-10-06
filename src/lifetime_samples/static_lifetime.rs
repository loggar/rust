// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// One special lifetime we need to discuss is 'static, which denotes that the
// affected reference can live for the entire duration of the program.
// All string literals have the 'static lifetime, which we can annotate as follows:

fn some() {
    let s: &'static str = "I have a static lifetime.";
}

// The text of this string is stored directly in the program’s binary, which is
// always available. Therefore, the lifetime of all string literals is 'static.

// You might see suggestions to use the 'static lifetime in error messages.
// But before specifying 'static as the lifetime for a reference, think about
// whether the reference you have actually lives the entire lifetime of your
// program or not, and whether you want it to. Most of the time, an error message
// suggesting the 'static lifetime results from attempting to create a dangling
// reference or a mismatch of the available lifetimes. In such cases, the solution
// is fixing those problems, not specifying the 'static lifetime.
