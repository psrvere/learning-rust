pub fn shadowing() {
    let x = 5;
    let x = x + 1; // this variable shadows above variable
    {
        let x = x * 2; // this variable shadows outer variable
        println!("The value of x in the inner scope is: {x}");
        // shadowing is limited within the inner scope
    }
    println!("The value of x is: {x}");
}