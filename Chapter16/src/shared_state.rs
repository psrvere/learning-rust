use std::{sync::Mutex, thread};
use std::rc::Rc;
use std::sync::Arc;

pub fn examples() {
    simple_mutex();
    sharing_mutex();
}

fn simple_mutex() {
    let m = Mutex::new(5);
    {
        // calling lock() will block the current thread until it 
        // acquires the lock
        // The call to lock() will fail if another thread holding the lock panics
        // Hence we have chosen .unwrap() so that this thread also panics if it
        // fails to acquire the lock
        let mut num = m.lock().unwrap();
        *num = 6;

        // The call to lock returns a smart pointer called MutexGuard
        // MutexGuard implements Deref to point at our inner data
        // It also implements Drop that releases the lock automatically when a MutexGuard
        // goes out of scope
        // As a result, we don't risk forgetting releasing the lock
    }

    println!("m = {:?}", m);
}

fn sharing_mutex() {
    // Approach 1
    // Using plain mutex counter will give us error that it has been moved
    // in the first iteration of for loop below
    // let counter = Mutex::new(0);
    
    // Approach 2
    // Wrapping mutex in Rc will give us another error
    // Rc<Mutex<i32>> cannot be sent between threads safely
    // let counter = Rc::new(Mutex::new(0));

    // Approach 3
    // Atomic Reference Counting: Arc<T>
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 1..10 {
        // Uncomment following line for Approach 2
        // let counter = Rc::clone(&counter);

        // Approach 3
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}