pub fn examples() {
    example1();
}

fn example1() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    
    {
        // Rc::clone doesn't make a deep copy. It only increments the referene count
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b: {}", Rc::strong_count(&a));
    }
    println!("count after b goes out of scope: {}", Rc::strong_count(&a));
    

    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c: {}", Rc::strong_count(&a));
}