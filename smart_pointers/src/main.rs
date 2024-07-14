use std::{ops::Deref, rc::Rc, cell::RefCell};
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
// Notes
// Boxes [Allocates data to heap] - for 2 thigs only
// 1) For variable with a trait type that cant be computed at compile time
// 2) For recursive data types - eg. a struct refernce to itself
// RcCell - RC->Refernce Count
// for multiple refernces
// use Rc::new for creating new , and Rc::clone(&ptr) for creating a new reference for existing item
// Rc::strong_count(&a) for getting count of refernces
// Arc<T> - same as RC but used for multi-threaded operations
// Cell provide intirior mutableity for types that impliment 'Copy' trait
// Cell methods ::new(val), .set(new_val), .get()

// For others [like strings] we use RefCell
// 
// RefCell - used for creating mutable 


// For having multiple ownership
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
  // refcell()
  reference_cycle()
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
    //clone has to be used to give the exact value of a without transferring ownership
    let b = Cons(Rc::new(RefCell::new(3)),Rc::clone(&a));
    let c =Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    

    *value.borrow_mut() +=10;
    // match b{
    //     Cons(data, next_address) => {
    //         match next_address {
    //             Cons(_, _) => todo!(),
    //             Nil => todo!(),
    //         }
    //     },
    //     Nil => todo!(),
    // }
    println!("a after ={:?}",a);
    println!("a after ={:?}",b);
    println!("a after ={:?}",c);

}  

fn reference_cycle(){
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    use List::{Cons,Nil};
     impl List{
        fn tail(&self)->Option<&RefCell<Rc<List>>>{
            match self {
                List::Cons(_, item) => Some(item),
                List::Nil => None,
            }
        }
     }

     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    
    print_loop(&a);
    fn print_loop(x: &List){
        match x {
            List::Cons(data, ref item) => {
              println!("data is {data}");
            //   print_loop(item.deref());
            },
            List::Nil => {
                println!("End of the line")
            },
        };
    }

}

