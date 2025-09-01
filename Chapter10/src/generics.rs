pub fn generics() {
    let number_list = vec![1, 2, 3, 4, 5];
    largest_number(&number_list);
}

fn largest_number(list: &[i32]) {
    let mut largest = &list[0];
    for num in list {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is: {largest}")
}

fn largest_number_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}