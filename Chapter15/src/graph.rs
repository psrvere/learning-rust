use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

pub fn weak_reference() {
    println!("*** Weak Reference ***");
    let leaf = Rc::new(Node{
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    // We can't print Weak<T> in a useful way. It just prints (Weak)
    // Hence we need to upgrade it to Rc<T> by calling upgrade method
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {

        let branch = Rc::new(Node{
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    } // branch goes out of scope, strong count goes to zero and memory is cleared
    // here weak count is still 1 but it has no bearing on dropping the branch data

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    // Moreover, without upgrade() we can't do anything useful with weak reference
    // Weak<T> just contains a pointer to the data, doesn't guarantee the data still exists
    // and can't access fields or methods
    // With upgrade we can access the data. Upgrade is needed for any operation on the data
    if let Some(parent) = leaf.parent.borrow().upgrade() {
        println!("Parent value: {}", parent.value);
        println!("Parent has {} children", parent.children.borrow().len());
    };
    // Note: parent goes out of scope here

    /* Q. How does creating Rc<T> from Weak<T> avoid reference cycles?
        upgrade() creates a TEMPORARY Rc<T>
        In above example, when parent goes out of scope, Rc is dropped
        Rc<T> from upgrade() is NOT stored
    */
}