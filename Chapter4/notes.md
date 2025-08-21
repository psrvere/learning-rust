# Chapter 4 - Understanding Ownership

It's a unique feature of Rust. It enables rust to make memory safety guarantees without needing a garbage collector.

In Some languages programmer must explicitly allocate and free the memory. In another languages we have garbage collector that regularly looks for no-longer-used memory as the program runs. Rust uses a third approach where memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile. None of the feature of ownership will slow down your program while it's running.

## The Stack and The Heap
The stack stores values in the order it gets them and removes the values in the opposite order. Last in, first out or First in, last out. Adding data is pushing on to the stack. Removing data is called popping off the stack. All data stored on the stack must have a known fixed size. Data with unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized. When you place data on heap you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of the location. The process is called allocating on the heap (pushing values on to stack is not allocating).

Pushing to the stack is faster than allocating on heap because in case of stack the allocator has to never search for a place to store data - that place is always on the top of the stack. Accessing data on the heap is slower than accessing data on the stack as you have to follow a pointer to get to the data.

When you call a function, the arguments and local variables of functions are pushed on to a stack. When function is over, these values are popped off.

Ownership addresses these probles:
- keeping track of what parts of code are using what data on the heap
- minimizing the amount of duplicate data on the heap
- cleaning up unused data on the heap so that you don't run out of space

## Ownership Rules
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped