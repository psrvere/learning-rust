// enum_and_struct will use enum to define IP type variant
// and use struct to store IP type and IP address
pub fn enum_and_struct() {
    // defining an enum
    #[derive(Debug)]
    enum IPAddrKind {
        V4,
        V6,
    }
    
    // How can we store IP Address and IP kind together?
    // We can use a struct
    
    #[derive(Debug)]
    struct IPAddr {
        kind: IPAddrKind,
        address: String
    }

    let home = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    dbg!(home);
}

// enum_only will use enum to define IP variant and store IP address
pub fn enum_only() {
    #[derive(Debug)]
    enum IPAddr {
        V4(String), // constructor function
        V6(String),
    }

    // we can attach data to each variant of enum directly
    // the name of each variant we define also becomes a function that contructs an instance of the enum
    // i.e. IPAddr::V4() is a function that takes String arg and returns
    // intance of IPAddr
    let home = IPAddr::V4(String::from("127.0.0.1"));
    dbg!(home);
}

// enum_only_v2 uses different types for each enum variant
// Enum variants can have different types or amount of associated data
pub fn enum_only_v2() {
    #[derive(Debug)]
    enum IPAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IPAddr::V4(127, 0, 0, 1);
    dbg!(home);
}

pub fn enum_variety() {
    enum Message {
        Quit, //  has not associated data
        Move{x: i32, y: i32}, // has named fields like struct
        Write(String), // has a string input
        ChangeColor(i32, i32, i32), //  has 3 i32 values
    }

    // Tradiotinally we would define these variants as 4 different structs:
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // turple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    // we would be able to easily define a function to take any of these kinds of messages as
    // we can define easilty with Message enum
    // Note: {} don't need to end with semicolon but unit struct and tuple structs () do need semicolon at the end.

    impl Message {
        fn call(&self) {
            // message body
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}