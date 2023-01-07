fn main() {
    //closures are anonymus funstions
    fn plus_one_v1 (x:i32)->i32 {x+1}
    let plus_one_v2  = |x:i32|  x+1;
    let plus_one_v3  = |x:i32|->i32 {x+1};
    let printing_closur = |x:i32|{
        println!("{}",x+2);
        x+2
    };
    assert_eq!(4,printing_closur(3),"Not true");
let answer = call_with_one(|x| x + 2);
assert_eq!(3, answer);
}

//closures as paramteres
fn call_with_one<F>(some_closure:F)->i32
where F:Fn(i32)->i32
{
     some_closure(32)
}

fn call_with_one_v2(some_closure: &dyn Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

// fn factory() ->&( dyn Fn(i32) -> i32 ){
//     let num = 5;
//     &|x| x + num
// }
