use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("Blue",10);
    let a = map.get("Blue").copied().unwrap_or(0);
    println!("{a}");
    let s = "hello world hi world";
    // ways to iterate over the maps
    for word in s.split_whitespace(){
        let count = map.entry(word).or_insert(0); // returns a pointer
        *count+=1;
    }
    println!("{map:?}");
    
    for (key, value) in &map{
        println!("{key}: {value}");
    }
}
