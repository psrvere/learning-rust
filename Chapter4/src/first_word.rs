// first_word functions objective is to returns the first word in a sentence

// this implementation first_word1 returns the index of first space in the sentence or
// the end of sentence in case there is only one word in the sentence

// &str type in parameter will accommodate both str and String type
// if we have a string slice (str), we can pass that directly
// if we have a String, we can pass a reference to the String, which will be a slice
// This takes advantage of "deref coercions"
pub fn first_word1(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len() // there is only one word
}

// first_word1 has a flaw. The original string can be changed before the caller can access/extract
// the sub word from the original string

// What is the problem?
// If we return index in this case, we have to worry about index in word getting out of sync with
// the data in s
// Hence we will look at String Slices Type!

pub fn string_slice() {
    let s = String::from("hello world");
    let slice = &s[0..2];
    println!("slice: {slice}"); // he
    println!("{}", &s[..2]); // he
    println!("{}", &s[..5]); // hello
    println!("{}", &s[6..]); // world
    println!("{}", &s[..]); // hello world
}

pub fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}

// We can continue to find and return second, third i.e. any sub segment of the sentence
// This is like an API call
// Rust compiler will ensure none of the returned slices go invalid by another part of code
// accidentally changing the original string

pub fn _string_literal() {
    let _s = "Hello World";
    // _s is a slice pointing to the place in binary where "Hello World" is stored
    // that's why string literals are immutable by design
}