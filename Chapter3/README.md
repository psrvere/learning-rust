This chapter covers common programming concepts of Rust


## Variables and Mutability
- By default variables are immutable. Can be made mutable with `mut` keyword
- constants are ALWAYS immutable
- Mut vs Shadowing
    - In shadowing we use let word again. Hence new variable is still immutable
    - In shadowing we create a new variable. We can change the type of variable in shadowing not in mut

## Data Types

- Rust is a statically typed language i.e. it must know types of all variables at compile time

### Scalar Types

- These are the primitive types in Rust

1. Integer Types
- Signed: i8, i16, i32, i64, i128, isize (arch)
- Unsigned: u8, u16, u32, u64, u128, usize (arch)
- Integer Literals: 100_000, 0xff, 0o644, 0b1100, b'A'
- Integer types default to i32

2. Floating Point Types
- f32 and f64
- default is f64 because on modern CPU is roughly the same speed as f32 but with more precision

Numeric Operations
- integer division truncates towards zero to the nearest integer

There is boolean type and char type also

### Compound Type

1. Tuple Type
- tuples have fixed length i.e. once declared these can't grow or shrink
- tuples can hold value of multiple types
- tuples can be destructured
- tuple values can also be accessed with tup.i notation where i is the index
- empty tuple `()` is also called a `unit`
- expressions implicitly return the unit value if they don't return any other value

2. Array Type
- every element must have the same type
- arrays in Rust have fixed length!
- arrays are useful when you want data allocation to happen in stack instead of heap or when you want to ensure a fixed number of elements
- there is another type `vector` type which behaves similar to array but can grow or shrink

### Functions
- In function signatures, you must declare the type of each parameter. This is a deliberate design decision in Rust to remove ambiguity for compiler.

Statements and Expressions
- Rust is an expression-based language
- Function bodies are made of a series of statements optionally ending in an expression
- Statements are instructions that perform some action and do not return a value
- Expressions evaluate to a resultant value
- In C & Ruby the assignment returns the value of assignment, not in Rust. `let x = (let y = 6)` is incorrect in Rust as `let y = 6` will not evaluate to anything
- `let y = 6`, here 6 is an expression which evaluates to 6. Numbers by themselves are expressions.
- calling a function is an expression
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // expressions do not end with semicolon. Adding semicolon here turns it into statement, then it will not return a value
    }; // this block will evaluate to 4, hence y value will be 4
}
```

Function with Return Values
- `fn five() -> i32` use `->` to declare return type
- the last expression is implicitly returned in functions
```rust
fn five() -> i32 {
    5 // the last expression will be implicitly returned
}
```

If Expressions
- the block of code in if/else expressions is called `arms`
- unlike JavaScript and Ruby evaluating a non-boolean expression will throw an error in if/else block
```rust
fn main() {
    let num = 3;

    if num { // This will throw error -  expected bool, found integer
        println!("num was three");
    } // note no semicolon required: it's an if block not a statement
}
```

Using If in a let Statement
```rust
let condition = true;
let number = if condition {5} else {6}; // here we have an if block which needs to be a statement so that its value can be assigned. Hence we used semicolon
```

Loops in Rust
- loop, for, while

loop:
- loop has no boolean condition to run the loop i.e. it always runs
- loop requires break statement to break
- loop can be labelled when there are nested loops to break a specific loop
```rust
'counting_up: loop = { // loop labels start with '
    break 'counting_up;
}
```

while:
- while has a boolean condition to start the loop

for:
- for loop brings safety over using while loop for iterating over an array. There are cases when using while loop can lead to out-of-bounds index errors
```rust
let a = [10,20,30,40,50];
for element in a {
    println!("the value is: {element}");
}
```