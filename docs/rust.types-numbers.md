# Rust numbers

## Integer Types

Integer Types in Rust

```
Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize
```

Integer Literals in Rust

```
Number literals	Example
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'
```

## Data types, the stronger the better

```rs
let variable_one: u32 = 16; // this is an unsigned 32bit integer.
let variable_two: char = 'c'; // this is a single character.
let variable_three: f64 = 3.14; // this is a 64bit floating point.
```

## Float

```rs
let num_var_one: f64 = 3.1415926; // PI is probably the most famous float
let num_var_two: f64 = 2.0; // even whole numbers must include at least one decimal place
```
