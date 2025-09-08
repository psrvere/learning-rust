pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

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

}