use std::{thread, time::Duration};

pub fn examples() {
    creating_thread1();
    creating_thread2();
    move_closure();
}

fn creating_thread1() {
    println!("*** creating_thread1 ***");
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    
    // In this example main thread will exit and child thread will be stopped
    // without ever completing all the loops
}

fn creating_thread2() {
    println!("*** creating_thread2 ***");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    // .join will block the main thread until spawned thread is finished
}

fn move_closure() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}