// example of bringing multiple items from one crate into scope
use std::{cmp::Ordering, iter};
// brings io and io::write into scope
use std::io::{self, Write};
// brings all items from crate into scope
use std::collections::*;

// private module, no code outside of this file can see it.
// inline definition. rustc will not look for front_of_house.rs or front_of_house/mod.rs
mod front_of_house {
    // public, code outside this module can see it.
    pub mod hosting {
        // public function
        pub fn add_to_waitlist() {}
        
        // private function
        fn seat_at_table() {} 
    }

    // private, no code outside the serving module can see it.
    mod serving {
        fn take_order() {}
        
        fn serve_order() {}
        
        fn take_payment() {}
    }
}

/* MODULE TREE *

crate
  |-- front_of_house
       |-- hosting
       |    |-- add_to_waitlist
       |    |-- seat_at_table
       |-- serving
            |-- take_order
            |-- serve_order
            |-- take_payment
*/

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // accesses the function deliver_order from parent.
        super::deliver_order();
    }
    
    fn cook_order() {}
    
    // Representing private and public in struct, fields are automatically private.
    pub struct Breakfast {
        // A customer picks their toast, but the chef picks the fruit-of-the-day.
        pub toast: String,
        seasonal_fruit: String,
    }
    
    // since enum is public, all variants are public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        // constructor needed since seasonal_fruit is private.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// brings back_of_house::Appetizer into scope for the entire file and any files 
// which bring this crate into scope. Called re-exporting.
pub use crate::back_of_house::Appetizer;

// Even though front_of_house is private, eat_at_restaurant can still access
// it since it is in the same file. 
// We call eat_at_restaurant and front_of_house as siblings.
pub fn eat_at_restaurant() -> back_of_house::Breakfast { 
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    
    // we bring Breakfast into scope with the 'use' keyword
    use back_of_house::Breakfast;
    // note, use only works in scope.

    // Order a breakfast in the summer with rye
    let mut meal = Breakfast::summer("Rye");
    // Change the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // we aren't allowed to change the fruit, though.

    meal
}

// using super in relative paths.
fn deliver_order() {}

// tests ran with cargo test
#[cfg(test)]
mod tests {
    // the as keyword changes the name of the item in scope.
    use super::eat_at_restaurant as eat;
    
    #[test]
    fn it_works() {
        let meal = eat();
        assert_eq!(meal.toast, String::from("Wheat"));
    }
}
