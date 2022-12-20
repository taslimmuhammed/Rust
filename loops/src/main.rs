fn main() {
   //n_loops()
  // while_loops()
  //for_each()
  range()
}

fn range(){
    //rev( function simply reversers the order)
    for number in (0..10).rev() {
        println!("{number}")
    }
}
fn for_each(){
    let a = ["tas10", "20", "30", "40", "50"];

    for element in a {
        println!("the value is: {element}");
    }
}
fn while_loops(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
fn n_loops(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");   
}