use std::{fs::{self, File}, io::{self, ErrorKind, Read}};

pub fn recoverable() {
    recoverable_error_match();
    recoverable_error_closure();
    println!("Result 1: {:?}", read_username_from_file());
    println!("Result 2: {:?}", read_username_from_file2());
    println!("Result 3: {:?}", read_username_from_file3());
    println!("Result 4: {:?}", read_username_from_file4());
}

fn recoverable_error_match() {
    let greeting_file_result = File::open("hello1.txt");
    
    // greeting_file_result will be an instance of Ok if file open was successful
    // Ok intance will have file handle contained in it
    // If file open fails, we will get instance of Err that has error information
    // Note: The Result enum and its variants have been brought into the scope
    // by the prelude

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello1.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("problem creating file: {:?}", e)
                }
            }
            other_error => {
                dbg!(&other_error);
                panic!("problem opening the file: {:?}", other_error)
            }
        }
    };
}

fn recoverable_error_closure() {
    let greeting_file = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening hte file: {:?}", error);
        }
    });

    // Other ways 1
    // Unwrap will handle the match logic under the hood
    // if file is found it will return OK instance
    // otherwise it will panic
    let greeting_file = File::open("hello2.txt").unwrap();

    // Other ways 2
    // We can add a custom message to panic
    let greeting_file: File = File::open("hello2.txt").expect("hello2.txt should be in the project");
}

//example for propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("user.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        // read_to_string returns usize which we are ignoring here
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Using ? for error propagation in rust
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("user.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Chaining method calls after ?
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("user.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// even shorter version 
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("user.txt")
}