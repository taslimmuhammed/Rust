extern crate modules_crates;

use modules_crates::english;
use modules_crates::japanese;

fn main() {
    println!("Hello in English: {}", english::greetings::hello());
    println!("Goodbye in English: {}", english::farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", japanese::farewells::goodbye());
}
