// Rust doesn't have inheritance
// But we can achieve something similar

// GUI Library Example

// In other programming languages we can define an interface with 
// methods like draw which components of GUI library to inherit
// Let's see how this is done in Rust

// Defining a trait for common behavior

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
// The Vector is of type Box<dyn Draw>, which is a trait object
// Any type that implements Draw can be added to this vector

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/* Q. What's the difference between defining components this way via trait objects vs
defining a struct that uses a generic type parameter with trait bounds?

A. A generic type parameter can be substituted with only one concrete type at a time. Whereas
trait objects allow for multiple concrete types to fill in for the trait object at runtime
*/

// Implementing Screen using generic type and a trait bound
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// This implementation if best for homogeneous collections

// Implementing Button Component

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw button
    }
}

// Implementing SelectBox Component

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw SelectBox
    }
}

// Implementing a screen example which includes above components

// Here you can see how we have used trait objects to store values
// of different types that implement the same trait
fn screen_example() {
    let screen = Screen{
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 50,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}