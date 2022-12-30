use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

enum List{
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons,Nil};
impl<T> Deref for MyBox<T>{
    type Target = T;
    
    //&Self::Target === &T
    fn deref(&self)->&Self::Target{
        &self.0
    }
}
fn main() {
    let x = 5;
//    let y = Box::new(x);

   //Box is a smart pointer which has deferencing trait
//    assert_eq!(x, *y);
   let y = MyBox::new(5);
   assert_eq!(x,*(y.deref()));
   let c = String::from("testing");
   drop(c); //callin the drop method on c

}

fn box_pointer(){
   let mut list = Cons(1,Box::new(Cons(2,Box::new(Cons(3, Box::new(Nil))))));
   let head = 
}
