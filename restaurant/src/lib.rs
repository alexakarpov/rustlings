#![allow(dead_code)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub const GREETING: &'static str = "Hallo, GREETIN is pub consts from lib.rs";

mod front_of_house {
    pub mod hosting {
        // note that both gotta be pub
        pub fn add_to_waitlist() {
            println!("adding to waitlist...");
        }

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {
            // super::deliver_order(); // FIXME
        }

        fn take_payment() {}
    }
}

pub fn deliver_order() {
    println!("deliver_order");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    use front_of_house::hosting;
    use front_of_house::serving;
    hosting::add_to_waitlist();
    crate::front_of_house::serving::serve_order();
    // Order a breakfast in the summer with Rye toast
    let mut meal = dbg!(back_of_house::Breakfast::summer("Rye"));
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
