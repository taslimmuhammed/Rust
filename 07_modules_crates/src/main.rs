extern crate modules_crates as phrases;

use phrases::english;
use phrases::japanese;

fn main() {
    println!("Hello in English: {}", english::greetings::hello());
    println!("Goodbye in English: {}", english::farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", japanese::farewells::goodbye());
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
