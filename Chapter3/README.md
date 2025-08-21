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
