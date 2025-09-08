pub fn examples() {
    example1();
    cons_list();
    simple_pointer();
    smart_pointer();
    my_box();
    custom_smart_pointer();
}

// The value of b is allocated on heap
// The data "5" is stored in the heap and a pointer to this data location
// is stored in the stack memory
fn example1() {
    let b = Box::new(5);
    println!("b = {b}")
}

// Enabling recursive types with Boxes
// Boxes don't have performance overhead
// Boxes provide Indirection and Heap Allocation
// Indirection means storing a pointer to value rather than value
// Box implements Deref Trait and Drop Trait
fn cons_list() {
    // List is a recursive type
    #[derive(Debug)]
    enum List {
        // use Box<T> to get a recursive type with a known size
        // Box<T> is a pointer and hence size is known to compiler
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(
        1, 
        Box::new(List::Cons(
            2, 
            Box::new(List::Cons(
                3, 
                Box::new(List::Nil)
            ))
        ))
    );
    dbg!(list);
}

fn simple_pointer() {
    let x = 5;
    let y = &x; // y points to value of x
    assert_eq!(5, x);
    assert_eq!(5, *y); // we can use deref operation on y to access value
}

fn smart_pointer() {
    let x = 5;
    let y = Box::new(x); // y points to copied value of x
    assert_eq!(5, x);
    assert_eq!(5, *y); // we can use deref operation on y to access value
}

fn my_box() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); 
    // Without Deref trait we get this error:
    // type `MyBox<{integer}>` cannot be dereferenced

    /*
        *y calls the deref method *(y.deref())
        y.deref() returns a pointer to the value
        why pointer? because we do not want to move ownership out of y
        then * does plain dereferencing to get i32 value here
        since i32 implements Copy trait, this copies the value
    */

    // Implicit Deref Coercion
    fn hello(name: &str) {
        println!("Hello, {name}");
    }

    let name = MyBox::new(String::from("Sam"));
    hello(&name);
    // Rust uses Deref Coercion to convert MyBox<String> type to &str
    // type for convenience of programmers. This is possible only because
    // MyBox<T> implements Deref trait
    // MyBox<String> --> &String --> &str
    // String type also implements Deref trait in standard library

    // if Rust didn't have deref coercion feature, then we will have to pass name
    // to the function like this:
    hello(&(*name)[..]);
    // *name = String
    // Since String doesn't implement copy trait, *name moves String out of MyBox
    // i.e. name is not invalid. This string is stored in a temporary variable
    // &String[..] = &str
    // Here hello gets a borrowed reference to string value
    // and the String is destroyed after the function call
}
    
fn custom_smart_pointer() {
    struct CustomSmartPointer {
        data: String,
    }

    // Rust doesn't allow us to call drop function manually
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomerSmartPointer with data `{}`", self.data);
        }
    }

    let c = CustomSmartPointer{
        data: String::from("some data"),
    };

    // But we can use std::mem::drop function which is part of prelude
    drop(c); // manually force dropping variable

    let d = CustomSmartPointer{
        data: String::from("some more data"),
    };
    println!("Custom pointers created");
}