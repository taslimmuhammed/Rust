
const three:i32 = 3;
static mut hello: &str = "hello world";
fn main() {
    //since the mutable static values can be accessed from any where ,
    //it'll cuase unsafety so it should be used inside the the unsafe block
    unsafe{
     println!("{hello}");
    }
   
}
