//declaring new vector
//-let v = vec![1, 2, 3]; //using vec! macro
//-let v: Vec<i32> = Vec::new();
//let mut v = Vec::new();  v.push(4)


//Accessing elements of vector
//-   v.get(2)
//-   v[4] 
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(5);
    let third = v[2];
    println!("The third element is {third}");
    
   match v.get(5){
     Some(fifth)=>println!("{fifth}"),
     None=> println!("Element not found")
   }

   for i in  &mut v {
     *i +=1;
     println!("{}",*i);
   }

}
