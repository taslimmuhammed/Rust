use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    // guess_number();
    let mut v = vec![107,2,3];
    let mut x = v[0]; // copies the value
    v.push(4);
    let mut y: Vec<&str> = Vec::new();
    y.push("hello");
    let z=  y[0];
    // let x = &v[0]; // Immutable reference to the first element
    // v.push(4);     // Error: Cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{z}")
}
fn guess_number(){
    let secret_no = rand::thread_rng().gen_range(0..=100);
    loop{
        println!("Guess your number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess:u32 = guess.trim().parse().expect("please neter a number");
        match guess.cmp(&secret_no){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it");
                break;
            }
        }
    }
    
    println!("secret number was {secret_no}");
}