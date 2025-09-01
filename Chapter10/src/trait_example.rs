use std::{fmt::Display, iter::Sum};

pub trait Summary1 {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // default implementation
    }
}

// Here objects need to implement only summarize_author 
// as summarize also uses summarize_author
pub trait Summary2 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author()) // calling another method
    }
}

pub struct NewsArticle1 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary1 for NewsArticle1 {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} {}",
            self.headline,
            self.author,
            self.location,
        )
    }
}

pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary2 for NewsArticle2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet1 {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary1 for Tweet1 {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait as Parameters

// impl trait syntax
// This will accept any type which has implemented Summary2 trait
pub fn notify1(item: &impl Summary2) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
// Above impl trait syntax is a syntactic sugar for following equivalent syntax
pub fn notify2<T: Summary2>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying multiple trait bounds with the + syntax
// In following functions, items must implement both Summary2 and Display traits
pub fn notify3(item: &(impl Summary2 + Display)) {}
pub fn notify4<T: Summary2 + Display>(item: &T) {}

// Clearer trait bounds with where clause
fn example1<T, U>(t: &T, u: &U) -> i32 
    where
        T: Display + Clone,
        U: Clone + Display,
{
    0 // fn not implemented
}

// Returning types that implements traits
// However we can only return one type which implements Summary2
// For example, if T1 and T2 implements Summary2
// A function with impl Summary2 as return signature
// it can either return T1 or T2 consistently in the function implementation
// but not both types
fn returns_summary() -> impl Summary2 {
    NewsArticle2{
        headline: String::from("India wins world cup!"),
        location: String::from("Mumbai, India"),
        author: String::from("Virat Kohli"),
        content: String::from("India once again wins the World Cup in 2025!..."),
    }
}

// Using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

// This conditionally implements for two traits
// T type has to implement Display and PartialOrd to be able to call
// cmp_display method
// Implementations of a trait on any type that satisfies the trait bounds are 
// called blanket implementations and are used extensively in the Rust standard library
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The larger number is {}", self.x);
        } else {
            println!("The larger number is {}", self.y)
        }
    }
}