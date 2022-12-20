struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); //tuple struct
struct AlwaysEqual; //unit struct


//test program
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&mut self,x:u32)->u32{
        self.width = 1000;
        self.width*self.height*x
    }
}
fn main() {
    let user = User{
        active:true,
        username:String::from("Taslim Muhammed Moosa"),
        email:String::from("taslimmuhammed67@gmail.com"),
        sign_in_count:1
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user
    };

    let black = Color(0, 0, 0);
    //can be accessed as black.0, black.1
    let sub = AlwaysEqual;

    let scale = 2;
    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("{}", rect1.area(100));
    // let area = area(&rect1);
    // println!("rect1 is {:?}, \n
    // rect1 is {:#?}", rect1, rect1); //{:?} debug train for developers wh want debug
     dbg!(&rect1);

}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
