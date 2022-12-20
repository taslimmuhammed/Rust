enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}



// there is no null in rust so you can use an Option<T> enum like the above


fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8), //strings, numeric types, structs, or enums can be included in the enums
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
   // println!("{:?}", loopback);

    let some_number = Some(5);
    let some_char = Some('e');

    match some_char {
        None => println!("No value here!"), // one match arm
        Some(x) => println!("Got value {}", x) // the other match arm
      };
      if some_char.is_some() {
        let real_value = some_char.unwrap();
        println!("unwrapped {}", real_value) ;
        //unwrap and except are two functions related to Option
      }

    if let Some(i) = some_char{
        println!("Got {}", i);
    }
   // let absent_number: Option<i32> = None;
   let none = plus_one(None);
   println!("{:?}",Coin::Quarter(UsState::Alabama));
   match_with_other();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn match_for_enums(coin: Coin) ->u32{
    match coin{
        Coin::Dime=> 1,
        Coin::Nickel=>2,
        Coin::Penny=>3,
        Coin::Quarter(state)=>{
            match state{
                UsState::Alabama=>4,
                UsState::Alaska=>5
            }
        }
    }
}

fn match_with_other(){
     let result = 23;
     match result{
        3=>println!("3 win"),
        7=>println!("7 win"),
        other=>println!("{other}"), //"_" can be used if you dont want to use the value of catched value
     }
}