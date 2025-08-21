# Chapter 4 - Understanding Ownership

It's a unique feature of Rust. It enables rust to make memory safety guarantees without needing a garbage collector.

In some languages programmers must explicitly allocate and free the memory. In other languages we have garbage collectors that regularly look for no-longer-used memory as the program runs. Rust uses a third approach where memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile. None of the features of ownership will slow down your program while it's running.

## The Stack and The Heap
The stack stores values in the order it gets them and removes the values in the opposite order. Last in, first out or First in, last out. Adding data is pushing on to the stack. Removing data is called popping off the stack. All data stored on the stack must have a known fixed size. Data with unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized. When you place data on heap you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of the location. The process is called allocating on the heap (pushing values on to stack is not allocating).

Pushing to the stack is faster than allocating on the heap because in the case of the stack the allocator never has to search for a place to store data - that place is always on the top of the stack. Accessing data on the heap is slower than accessing data on the stack as you have to follow a pointer to get to the data.

When you call a function, the arguments and local variables of the function are pushed onto the stack. When the function is over, these values are popped off.

Ownership addresses these problems:
- keeping track of what parts of code are using what data on the heap
- minimizing the amount of duplicate data on the heap
- cleaning up unused data on the heap so that you don't run out of space

## Ownership Rules
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## Variable Scope
It's the same as other programming languages. Variable scope starts when a variable is defined and ends at the end of that scope - function scope, block scope, etc.

## The String Type
Consider this example
```rust
fn main() {
    // :: is the namespace operator
    // from is an associated function (similar to static method in other languages)
    // Associated Function use ::
    // Whereas Methods use .
    // Associated Function are called on the type, not the instance
    // Methods are called on an instance
    let mut s = String::from("Hello");
    s.push_str(" World!");
    println!("{s}")
}
```
Q. How come String can be mutated but string literals can not be?

Memory and Allocation
In the case of string literal we know the contents at the compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. This won't be possible if string literals were mutable.

With the String type, in order to support a mutable, growing piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.

This is a two step process:
- First, we need request memory from memory allocator at runtime
- Second, we need a way to return this memory to the memory allocator when we are done

Q. What kind of problems do programmers run into when they allocate and free memory manually?
- programmers forget to free memory - wastes space
- if we do it too early, we have an invalid variable
- if we free memory twice, that's a bug too

First Step: When we define a String `let mut s = String::from("Hello")`, the implementation of `String::from` requests the memory it needs. This concept is universal across programming languages.

Second Step: In Rust, the memory is automatically returned once the variable that owns it goes out of scope. Rust does this by calling a special function called `drop`. 

Let's look at some examples where this pattern can be helpful:

1. Variable and Data interacting with Move
Integer variables:
```rust
let x = 5 // integers are simple values with known fixed size. This is stored in stack
let y = x // another 5 value is stored in stack
```

String variables
```rust
let s1 = String::from("hello")
let s2 = s1
```
Q. How is s1 stored?
String Data is made up of three parts
- a pointer to the memory which stores string content
- length
- capacity
One stack stores pointer, length and capacity. The contents of string (that pointer points to), each character in one slot, is stored in heap

Q. How is s2 stored?
s1 Data (pointer, len and capacity) is copied to another stack but heap data is not copied i.e. the pointers of s1 and s2 point to the same underlying heap data.

Q. What is a double free error?
In above examples, both s1 and s2 will try to free memory when they go out of scope. Rust handles this by invalidating s1 when s2 is defined to ensure memory safety. See this example:
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("s1: {s1}") // this will throw error: borrow of moved value: s1
```
This is similar to concept of `shallow copy` in other programming languages but with a twist - Rust also invalidates the first variable. It is called `a move` i.e. s1 was moved into s2.

There is an implied design choice here: Rust will never automatically create "deep" copies of your data i.e. any automatic copy can be assumed to be inexpensive in runtime.