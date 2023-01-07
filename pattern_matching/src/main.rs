fn main() {
    //even let statement is a pattern matching 
   // while_let()
   //for_loop()
   let point = (3, 5);
    print_coordinates(&point);
}

struct Point {
    x: i32,
    y: i32,
}

fn at_binding(){
    enum Message{
        Hello{
            id:i32
        }
    }

    let msg =  Message::Hello { id: 3 };

    match msg {
        Message::Hello { id:id_variable@3..=7 } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
fn conditional_matching(){
    let num =Some(4);
    let x=6;
    match num{
        Some(x) if x%2==0 =>println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}
fn range_ignoring(){
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

}
fn destructuring(){
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    //OR
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    //macthing
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
fn range_matching(){
    let x= 5;

    match x{
        1..=5 =>println!("between 1 and 5"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
fn matching_litrels(){
    let x=1;
    match x{
        1|2 => println!("One or Two"),
        3=>println!("three"),
        _=>println!("other number")
    }

    match x{
        1|2 => println!("One or Two"),
        3=>println!("three"),
        y=>println!("number is {y}")
    }

    let x = Some(5);
    let y=10;

    match x {
        Some(50)=>println!("we got 50 : {}",x.unwrap()),
        Some(y)=>println!("We have got {y}"),
        _=>println!("other number")
    }
}
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn for_loop(){
    let v = vec![1,2,3];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
fn while_let(){
    let mut v1 = Vec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);

    while let Some(x) = v1.pop(){
        println!("{x}");
    }
}
fn if_let(){
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    //if let statments work if the patterns match
    //for eg if None was here it would'nt work
    //if we if let x=5, it'll always work and we can use x insode the parenthesis
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } 
}