#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use rand::Rng;
// nested paths
use std::{io, cmp::Ordering};

//
// use std::io;
// use std::io::Write;
// ===> 
// use std::io::{self, Write};

// also, a global operattor imports all public itesm into scop
// use std::collections::*;


/**
 * Paths can take two forms:
 * 1. absolute path: starts from a crate root by using a crate name or a literal crate
 * 2. relative path: starts from current module and use self,super, or an identifier 
 *      in the current module
 */
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}


mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // 使用super访问
        super::serve_order();
        // using super to route path... 
        super::front_of_house::hosting::add_to_waitlist();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    eat_at_restaurant2();
    hosting::add_to_waitlist();
    // 
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // use absolute path to refer mod
    // crate, root of crate's module tree.
    crate::front_of_house::hosting::add_to_waitlist();
    
    // relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // the next line whon't compile if we uncomment it, we are not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("bluebeeries");
}

// pub 
use crate::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}