mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    // In structs we have to choose which fields we want to make public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Note: since one of the field of Breakfst struct is private, we need to provide
    // at least one public contructor function to get new instances of Breakfast
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    // On the other hand, if we make an enum public, all its variants are then public
    // the design choice is because enums aren't very usefull unless all their variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // we can use super to refer to the parent module. In this case it is crate/root module.
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}


use crate::front_of_house::hosting; // this will allow us to call hosting directly

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    // SInce eat_at_restaurant and front_of_house are siblings, crate can be ommited in the path
    front_of_house::hosting::add_to_waitlist();

    hosting::seat_at_table(); // calling hosting directly
}