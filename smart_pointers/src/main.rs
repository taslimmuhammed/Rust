use std::{ops::Deref, rc::Rc, cell::RefCell};
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

// enum List{
//     Cons(i32, Box<List>),
//     Nil
// }

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
   drop(c); //calling the drop method on c
  // reference_counter()
   refcell()
}


fn reference_counter(){
    enum List{
        Cons(i32, Rc<List>),
        Nil
    }
    use List::{Cons,Nil};

    let a= Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3,Rc::new(Nil)))))));
    let b= Cons(4, Rc::clone(&a));
     //a.clone can also be used here
    {
        let c =Cons(5, Rc::clone(&a));
        println!("No of pointers to a is :-{}",Rc::strong_count(&a));
    }
    //Rc keeps track of all the pointers pointing towards "a"
    //And drops the value only when there are no pointers
    println!("No of pointers to a is :-{}",Rc::strong_count(&a));
}


fn refcell(){

    //Rc=>To have multiple pointers towards it
    //RefCell=>To have mutable values
    #[derive(Debug)]
    enum List{
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil
    }
    use List::{Cons,Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)),Rc::clone(&a));
    let c =Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    

    *value.borrow_mut() +=10;
    // match b{
    //     Cons(data, next_address) => {
    //         match next_address::List {
    //             Cons(data2, _) =>{},
    //             Nil => {},
    //         }
    //     },
    //     Nil => todo!(),
    // }
    println!("a after ={:?}",a);
    println!("a after ={:?}",b);
    println!("a after ={:?}",c);

}  

