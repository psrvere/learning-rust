#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// shoes_in_size takes ownership of both params.
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() then takes ownership of shoes param
    // filter closure captures shoe_size from the environment
    // collect consumes the iterator and returns a vector of Shoe
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe{
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 13,
                style: String::from("sandal"),
            },
            Shoe{
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
            Shoe{
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 10,
                style: String::from("boot"),
            },
            ]);
    }
}