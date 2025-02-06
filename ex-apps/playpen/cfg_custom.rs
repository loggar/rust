// Some conditionals like target_os are implicitly provided by rustc, but custom conditionals must be passed to rustc using the --cfg flag.
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}

/*
rustc --cfg some_condition ./ex-apps/playpen/cfg_custom.rs --out-dir ./target.ex-apps/playpen && \
./target.ex-apps/playpen/cfg_custom

condition met!
*/

/*
rustc ./ex-apps/playpen/cfg_custom.rs --out-dir ./target.ex-apps/playpen && \
./target.ex-apps/playpen/cfg_custom

error[E0425]: cannot find function `conditional_function` in this scope
 --> ./ex-apps/playpen/cfg_custom.rs:8:5
  |
8 |     conditional_function();
  |     ^^^^^^^^^^^^^^^^^^^^ not found in this scope

error: aborting due to 1 previous error
*/
