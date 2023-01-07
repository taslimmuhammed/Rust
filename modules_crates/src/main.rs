extern crate modules_crates as phrases;

use phrases::english;
use phrases::japanese;

fn main() {
    println!("Hello in English: {}", english::greetings::hello());
    println!("Goodbye in English: {}", english::farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", japanese::farewells::goodbye());
}
