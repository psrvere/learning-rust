use std::cell::RefCell;

// The Messernger design ensured safety
// self is immutable so that it is not accidently changed
// Using &self allows us to have multiple immutable references
// Using &mut self allows us to have only one mutable reference
// In this context, using &self is a better design
pub trait Messenger {
    fn send(&self, msg: &str);
}

// Here T must implement Messenger trait
// T: Messenger also implies that this struct can only be created by types
// which implement Messenger trait
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

// Here T: Messenger implies this implementataion only works for the type
// that implements Messenger trait
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { 
            messenger, 
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota.");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: You are at 90% of your quota.");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You are at 75% of your quota.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger1 {
        sent_messages: Vec<String>,
    }

    impl MockMessenger1 {
        fn new() -> MockMessenger1 {
            MockMessenger1 { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger1 {
        fn send(&self, message: &str) {
            // This will show Error - uncomment and check.
            // cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
            // `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
            // What this error means is that since &self is immutable reference
            // &self.sent_messages return an immutable referecne to Vec
            // hence push method can not work on it
            // Note: we have already established above why &self is a better design 
            // over &mut self

            // This is a situation where interior mutability can help!
            // See second example below

            // self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    #[ignore = "This test won't work"]
    fn it_sends_an_over_75_percent_warning_message1() {
        let mock_messenger = MockMessenger1::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger,
            100,
        );

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.len(), 1)
    }


    // Using RefCel<T> for interior mutability
    use std::cell::RefCell;
    struct MockMessenger2 {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger2 {
        fn new() -> MockMessenger2 {
            MockMessenger2 { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger2 {
        fn send(&self, message: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message2() {
        let mock_messenger = MockMessenger2::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger,
            100,
        );

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1)
    }

    /* Keeping track of borrows at Runtime with RefCell<T>
        borrow method returns the smart pointer type Ref<T>
        borrow_mut method returns the smart pointer type RefMut<T>
        Both types implement Deref, hence we can treat them like regular references
        RefCell<T> keeps count of these references and ensures many immutable borrows
        OR only one mutable borrow at a given time
        Violating this rule, will make program panic at runtime
    */
}

pub fn examples() {
    multiple_owners_mutable();
}

// Allowing multiple owners of mutable data with Rc<T> and RefCell<T>
fn multiple_owners_mutable() {
    use std::rc::Rc;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    let value = Rc::new(RefCell::new(5));
    
    // We need to clone value so both a and value have ownership of the inner 5 value
    // rather that transferring ownership from value to a or having a borow from value
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}