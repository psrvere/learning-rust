pub fn examples() {
    example1();
    example5();
}

fn example1() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    for val in v_iter {
        println!("Value: {val}");
    }
}

#[test]
fn example2() {
    let v = vec![1, 2, 3];
    // iter() doesn't take ownership of v. Hence it returns immutable references
    let mut v_iter = v.iter();

    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
}

#[test]
fn example3() {
    let v = vec![1, 2, 3];
    // into_iter() will take ownership of v and will return owned values
    let mut v_iter = v.into_iter(); 

    assert_eq!(v_iter.next(), Some(1));
    assert_eq!(v_iter.next(), Some(2));
    assert_eq!(v_iter.next(), Some(3));
    assert_eq!(v_iter.next(), None);
}

// Methods that consume an iterator
// Consuming Adapters
#[test]
fn example4() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    let total: i32 = v_iter.sum(); // sum takes ownership of v. Example of a consuming adapter
    assert_eq!(total, 6);
}

// Methods that produce other iterators
// Iterator Adapters
fn example5() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    let v1: Vec<_> = v_iter.map(|x| x + 1).collect();
    for val in v1 {
        println!("val: {val}");
    }
}
