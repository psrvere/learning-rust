use std::{sync::mpsc, thread, time::Duration};
// mpsc = multiple producer, single consumer       

pub fn examples() {
    using_channels();
    send_multiple();
    multiple_producers();
}

fn using_channels() {
    println!("*** using_channels ***");
    let (tx, rx) = mpsc:: channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // this will move val to receiver's thread
        // Hence printing val below will throw compiler error
        // println!("val is {val}");

        // First, send function takes ownership of val
        // When receiver receives the value, it takes ownership i.e. val has moved
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn send_multiple() {
    println!("*** send_multiple ***");
    let (tx, rx) = mpsc:: channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn multiple_producers() {
    println!("*** multiple_producers ***");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("tx1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}