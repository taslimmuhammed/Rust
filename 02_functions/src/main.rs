fn main() {
    // another_function(5);

    // let y = {
    //     let x = 3;
    //     x + 1 //this line is not a statment it returns the value
    // };

    // println!("The value of y is: {y}");
    // let five = five(7);
    // println!("The five :- {five}")
    ifTest()
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five(x:i32) -> i32 {
    5+x
}


fn ifTest(){
    let condition = true;
    let number = if condition { 5 } else { 6 };

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2 //returns this value as result ie. 2*10=20 
        }
    };

    println!("The result is {result}");
}