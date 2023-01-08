fn main() {
     let fcn = return_closure();
     let x = fcn(2);
     println!("{x}");
}
fn add_one(x:i32)->i32{
    x+1
}

fn mapping(){
    let v1 = vec![1,2,3];
    let list_of_strings :Vec<String> = v1.iter().map(|i| i.to_string()).collect();

    //the same function can be done like 
    let list_of_strings  :Vec<String>= v1.iter().map(ToString::to_string).collect();
    
    enum Status{
        Value(u32),
        Stop
    }
    let list_of_statuses :Vec<Status> = (0u32..20).map(Status::Value).collect();

}
//functions as parameters
fn do_twice(f:fn(i32)->i32,x:i32)->i32{
    f(x)+f(x)
}

//we have to wrap the closure in the smart pointer or return its address
//becuase its size is not known at the compile time
fn return_closure()->&'static dyn Fn(i32)->i32{
   if true {
    & |x| x+1
    }else{
    & |x| x+1
    }
}


fn return_closure_2()->Box<dyn Fn(i32)->i32>{
    Box::new(|x| x+1)
}

