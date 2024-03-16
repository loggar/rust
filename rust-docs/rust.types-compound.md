# Rust Compound Types

## Tuples

```rs
// Declare the tuple
let tuple_variable: (f64, char, i32) = (3.1415, 'c', -7);
// Prints: float 3.1415, character c, integer: -7
println!("float {}, character {}, integer: {}", tuple_variable.0, tuple_variable.1, tuple_variable.2);

// Destructure the tuple
let (float, character, integer) = tuple_variable;
// Prints: float 3.1415, character c, integer: -7
println!("float {}, character {}, integer: {}", float, character, integer);
```

## Arrays

```rs
// Declare the array
let array_variable: [char; 3] = ['a', 'b', 'c'];
// Prints: a, b, c
println!("{}, {}, {}", array_variable[0], array_variable[1], array_variable[2]);

// Destructure the array
let [a, b, c] = array_variable;
// Prints: a, b, c
println!("{}, {}, {}", a, b, c);
```
