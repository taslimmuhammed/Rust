extern "C" {
    fn abs(input: i32) -> i32;
}
//extern is the way rust communictes with external code


fn main() {
    unsafe_pointers();
    unsafe{
        println!("calling external fucntion {}", abs(-3));
    }
}

unsafe fn unsafe_function(){
    //unsafe functions can only be called in unsafe block
    //usually used to imteract with unsafe Languages like C
}
fn unsafe_pointers(){
    let mut num = 4;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
         *r2 =5;
    println!("{num}");
    }
   
}

//tells compiler to not mangle
//Mangling is when a compiler changes the name we’ve given a function to a different name
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
unsafe trait Foo {
    // methods go here
    //when atleast one of its methods is unsafe
}
