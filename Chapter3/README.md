This chapter covers common programming concepts of Rust


## Variables and Mutability
- By default variables are immutable. Can be made mutable with `mut` keyword
- constants are ALWAYS immutable
- Mut vs Shadowing
    - In shadowing we use let word again. Hence new variable is still immutable
    - In shadowing we create a new variable. We can change the type of variable in shadowing not in mut

## Data Types

- Rust is a statically types language i.e. it must know types of all variables at compile time

### Scaler Types

- These are the primitive types in Rust

1. Integer Types
- Signed: i8, i16, i32, i64, i128, isize (arch)
- Unsigned: u8, u16, u32, u64, u128, usize (arch)
- Integer Literals: 100_000, 0xff, 0o644, 0b1100, b'A'
- Integer types default to i32

2. Floating Point Types
- f32 and f64
- default is f64 because on modern CPU is roughly the same speed as f32 but with more precision

Numeric Oprations
- integet division truncates towards zero to the nearest integer

There is boolean type and char type also

### Compound Type

1. Tuple Type
- tuples have fixed length i.e. once declared these can't grow or shrink
- tuples can hold value of multiple types
- tuples can be destructured
- tuple values can also be accessed with tup.i notation where i is the index
- empty tuple `()` is also called a `unit`
- expressions implicitly return the unit value if they don't return any other value

2. Arrray Type
- every element must have the same type
- arrays in Rust have fixed lenght!
- arrays are useful when you want data allocation to happen in stack instead of heap or when you want to ensure a fixed number of elements
- there is another type `vector` type which behaves similar to array but can grow or shrink