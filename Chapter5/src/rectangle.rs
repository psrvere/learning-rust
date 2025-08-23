#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// defining methods on Rectangle
// Self (with uppercase S) is type alias for the struct type i.e. Self = Rectangle
// self (with lowercase s) is parameter name. It represents the instance being called on
// &self is reference to the instance
// other self patterns:
// area(self)
// area(&mut self)
impl Rectangle {
    // square is not a method as it does not have self as the first parameter
    // String::from() is another example of a function which is not a method
    // These are often used for constructors and often called new. new is not a reserved name in Rust
    // this needs to be called like this Rectangle::square()
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // we can define a method with same name as a struct field
    fn width(&self) -> bool {
        self.width > 0
    }

    // another method to check if self Rectangle can hold another Rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
/*
All functions defined in impl block are called Associated Functions
because they are associated with the type on which mehtods are defined
*/

pub fn calculate_area() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // since dbg returns ownership of value, this expression is the same
        // as assigning 60 value to width
        height: 80
    };

    let rect2 = Rectangle {
        width: 10,
        height: 20
    };

    let sq = Rectangle::square(30);
    println!("square: {:#?}", sq);

    // new implementation using methods
    println!("area: {}", rect1.area());

    // this is same as above line. How come?
    // Rust has a feature called automatic referencing and automatic dereferencing
    // When we call a method, Rust automatically adds in &, &mut or * so objects matches
    // the signature of the method
    // Rust makes borrowing implicit for method receivers - this makes ownership ergonomic in practice!
    println!("area: {}", &rect1.area());

    // when we use width without (), Rust knows it's a field
    println!("width field: {}", rect1.width);

    // when we use width with (), Rust looks for a method
    println!("width method: {}", rect1.width());

    println!("can rect1 hold rect2?: {}", rect1.can_hold(&rect2));

    // older implementation without using methods
    // println!("area: {}", area(&rect1));
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

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }