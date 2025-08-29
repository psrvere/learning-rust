
pub fn text() {
    // Strings are implemented as collection of bytes

    /* Q. What is a string in rust?
        1. String Slice: &str (usually seen in borrowed form) - reference to some UTF-8 encoded data stored elsewhere
        This is implemented in the rust core language
        2. String type: provided by Rust standard library. It is a growable, mutable, owned, UTF-8 encoded string type
    */

    // String is implemented as a wrapper around a vector of bytes

    // create new empty string
    let mut s = String::new();
    s.insert(0, 'a');
    println!("empty string: {s}");

    // create new string from string literal - Method 1
    let s = "hello world".to_string();
    println!("string: {s}");

    // create new string from string literal - Method 2
    let s = String::from("hello again!");
    println!("string: {s}");

    // Updating a string

    // push_str method
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("string: {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar"; 
    s1.push_str(s2); // push_str takes a string slice as we don't want to take ownership of the string
    // i.e. s2 will be available after using it with push_str method
    println!("s1: {}, s2: {}", s1, s2);

    // push method
    // It takes a single character and add it to string
    let mut s = String::from("hell");
    s.push('o');
    println!("string push: {s}");

    // Combining two strings

    // Using + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // Note: s1 has been moved here and hence cannot be used
    println!("s3: {s3}");
    // The + operator calls add function
    // fn add(self, s: &str) -> String {}
    // Q. But s2 is String type so how come add function accepts it?
    // Rust uses `deref coercion` which turns &s2 into &s2[..]

    // Using format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // doesn't take ownership of any of its parameters
    print!("s: {s}");

    // Rust strings DO NOT support indexing!
    // s[0] will throw compile error

    let s = String::from("नमस्ते");
    println!("bytes: {:?}",s.as_bytes());
    // This will print byte representation of string
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    // Methods for iterating over string

    // Method 1: chars()
    for c in s.chars() {
        println!("character: {c}");
    }
    /*
        This will print unicode scalar values:
        character: न // letter
        character: म // letter
        character: स // letter
        character: ् // diacritic
        character: त // letter
        character: े // diacritics

        All letters together are called grapheme cluster.
    */

    // Method 2: bytes()
    for b in s.bytes() {
        println!("bytes: {b}")
    }
}