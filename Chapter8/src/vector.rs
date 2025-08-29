pub fn vector() {
    // new empty immutable vector
    let v1: Vec<i32> = Vec::new();
    dbg!(v1);

    let v2 = vec![1, 2, 3];
    dbg!(v2);

    // new empty mutable vector
    let mut v3: Vec<i32> = Vec::new();
    v3.push(1); // adding values to a vector
    dbg!(v3);

    // reading data from vectors
    let v4 = vec![1,2,3,4];

    // first way is reading element is via indexing
    // Since we want to just read the value, we will avoid copying by borrowing value
    let third: &i32 = &v4[2];
    println!("thirds: {third}");

    // second way of reading is via the get method which returns value as Option<&T>
    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("third: {third}"),
        None => println!("there is no third element."),
    }

    // Let's try to access an element out of range with these methods

    // this piece of code will panic
    // error: index out of bounds: the len is 4 but the index is 4
    // let fifth: &i32 = &v4[4];
    // println!("fifth: {fifth}");

    // this piece of code will not panic
    let fifth: Option<&i32> = v4.get(4);
    match fifth {
        Some(fifth) => println!("fifth: {fifth}"),
        None => println!("there is no fifth element"),
    }

    let mut v5 = vec![1,2,3,4];
    let first = v5[0];
    // v5.push(6); // Error: cannot borrow `v5` as mutable because it is also borrowed as immutable
    dbg!(first);

    // Question - the new element will be added at the end of the vector, so why does rust care
    // if we have readable reference to the first element?
    // Answer - since values of vector are kept next to each other, adding a new element might
    // require rust to reallocate all the values to a new area and then the reference to the first
    // element, will point to a deallocaed memory; the borrowing rules prevent this 
    // from happening

    // Iterating over the values in a Vector

    // Example: immutable vector
    let v6 = vec![10, 20, 30];
    // Here we have to borrow vector for the for loop, otherwise, it will consume vector 
    // and it will be moved i.e. won't be available in this function after for loop
    for val in &v6 {
        println!("{val}");
    }

    // Example: mutable vector
    let mut v7 = vec![10, 20, 30];
    for val in &mut v7 {
        *val += 1;
    }
    dbg!(v7);

    // Using an Enum to Store Multiple Types    
    // Vectors can only store values that are of the same type. 
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.14),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    dbg!(row);
}