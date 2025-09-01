// pub fn dangling_ref() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     } // x will be out of scope here
//     println!("r: {r}");
// }
// This will throw error that x does not live long enough
// Rust compiler uses Borrow Checker to check this

// Above function can be fixed like this
pub fn dangling_ref_fixed() {
    let x = 5;
    let r = &x;
    println!("r: {r}");
}

// Generic Lifetimes in Function
pub fn generic_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz"; // This is static lifetime i.e. it can live
    // for the duration of program. The text is stored directly in the binary.
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// Original function without lifetimes which throws error
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Added explicit lifetimes here
// The return reference is valid as long as both x and y are valid
// x, y and return reference all live at least 'a lifetime
// this is an additional constraint we have defined for borrow checker
// 'a will be smaller of the lifetime value of x and y - which is same in this case
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// We can not control lifetimes of a variable created inside the function
// Its lifetime is limited to end of function or a smaller scope within the function


// Lifetime annotation in struct definitions
// Structs with reference types will need lifetime to be defined
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Elision Rules

// First Rule: Assign a lifetime parameter to each reference parameter
// Second Rule: If there is exactly one input lifetime parameter, then that lifetime
// is assigned to all output lifetime parameters
// Third Rule: If there are multiple input lifetime parameters, but one of them is
// &self or &mut self (it's a method), the lifetime of self is assigned to all 
// output lifetime parameters

