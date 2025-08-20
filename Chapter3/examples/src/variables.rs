pub fn variables() {
    let x = 5;
    // let mut x = 5; // This will not throw compile error as x is now mutable
    println!("The value of x is: {x}");
    x = 6; // Error: cannot assign twice to immutable variable
    println!("The value of x is: {x}");
}