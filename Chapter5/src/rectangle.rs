#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn calculate_area() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // since dbg returns ownership of value, this expression is the same
        // as assigning 60 value to width
        height: 80
    };
    println!("area: {}", area(&rect1));
    // Note: &rect1 is a borrowed struct
    // Accessing its fields in area function does not move the fields values i.e. struct
    // still has ownership of these fields
    // this is a common pattern in Rust with structs

    // Let's try printing struct
    // println!("rect1: {}", rect1); // Error: `Rectangle` doesn't implement `std::fmt::Display`
    // the primitive types implement `Display` (it has how to format logic) which println can 
    // use to show these values in a particular format.

    // What is the Solution?
    // use `Debug` formatting using {:?}
    // and add #[derive(Debug)] we are telling Rust to opt in to the functionality to print out 
    // debugging information
    println!("rect1: {:?}", rect1);

    // another format which will print struct fields in new lines like a JSON
    println!("rect: {:#?}", rect1); 

    // println! vs dbg!
    // println! takes a reference of the expression and prints to stdout
    // dbg! takes ownership of an expression, prints to stderr, prints file and line number too,
    // and returns the ownership of the value

    dbg!(&rect1); // we don't want dbg to take ownership of rect1, hence we passed a reference
    // if we do not use ; in the above line, then it makes it an expression which will return
    // reference to rect1
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}