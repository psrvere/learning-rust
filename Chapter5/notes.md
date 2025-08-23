# Using Structs to Structure Related Data

## Defiing and Instantiating Structs
- Rust doesn't allow us to mark only certain fields as mutable, the entire instance must by mutable

## Using the field init Shorthand
- instead of writing `email: email` while instantiating a strcut, just write `email` for email field

## Struct Update Syntax
We can create new structs with other existing struct using
```rust
let user1 = User {
    active: true,
    username: Strings::from("Sam"),
    email: Strings::from("sam@gmail.com"),
    sign_in_count: 1,
}

let user2 = User {
    email: String::from("jack@gmail.com");
    ..user1 // this will fetch remaining values from user1 struct. This should be last to specify.
};
// IMP: In this example we can no longer use user1 as the username filed of user1 has moved to username field for user2. If we had given user2 string values for both username and email i.e. we used only active and sign_in_count fields from user1, in this case, user1 will still be valie
```
I like this, it reduces a lot of reduntant work, especially while working on CRUD

- Tuple structs can be used to define a type. All the values in the tuple are of the type: tuple-name

Using owned types vs reference in struct fields:
- in User struct email is defined as owned String type. The choice is made because we want the struct to own the data as long as stuct lives i.e. the fields data ownership is with the struct
- we can define refernece to data owned by something else in struct fields usin lifetimes - chapter 10
- if we try to assign email as &str type instead of String type, we will get compiler error: missing lifetime specifies