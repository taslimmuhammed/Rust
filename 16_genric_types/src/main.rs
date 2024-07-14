struct Point<T,U>{
    x:T,
    y:U
}

impl <T, U> Point<T,U>{
    fn mixup<V,W>(self, other: Point<V, W>)->Point<T,W>{
      Point { x: self.x, y: other.y }
    }
}


//we can define function for just a type
impl Point<f64,f64>{
    fn y(&self)->f64{
        self.y
    }
}
fn main() {
    let v = vec!["1","2","3","a"];
    let chr =  get_largest(v);
    println!("{chr}");
    let p  = Point { x:"h", y:"y" };
}


fn get_largest<T: PartialOrd+Copy>(number_list:Vec<T>) -> T{
    let mut largest = number_list[0];
    for number in number_list{
        if number >largest{ largest = number;}
    }
    largest
}