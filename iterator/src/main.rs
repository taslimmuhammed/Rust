
struct Shoe{
    size:u32,
    style:String
}


//filter using closure
fn filter_shoe_in_size(shoes:Vec<Shoe>, size:u32)->Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size== size).collect()
}
fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for value in v1_iter{
        println!("{value}")
    }

    let v2:Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("{:?}",v2);
    //iter methods
    //sum=:- gives the sum of all the results
    //skip:- used to skip n number in the begenning
    let v3 :u32= v2.iter().skip(1).sum();
    println!("{:?}",v3);
} 
