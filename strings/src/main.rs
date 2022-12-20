fn main() {
    let mut stri= String::from("hello");
    printstr(stri);
    //println!("{stri}") 
    //here it will fail cuase the str value dropped by the printstr function which sent the exact thing instead of a copy
    let mut s = String::from("hello world");
    //change(&mut s);
    //println!("{s}") ;
    //mutiple_mutations(&mut s);
    // let length = first_word(&s);
    // println!("{length}");
    // s.clear(); // this empties the String, making it equal to ""}
    //cpy_parts(&s);
    let word = first_word(&s[0..5]);
}

fn cpy_parts(s:&String){
    let hello = &s[0..5];
    let world = &s[6..];
    let slice = &s[..];
    println!("{hello}, \n{world},\n{slice}")
}
// str => string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

//enumerate wraps the ressult and gives it as tuple(index, &char)
//iter alone just convert it to chars
for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
        return &s[0..i];
    }
}

&s[..]
}

fn mutiple_mutations(s2: &mut String) {
    println!("{s2}");
    let mut s = String::from("hello");
{
    let _r1 = &mut s;
    //let r2  = &mut stri;
    // here r2 cannot be declared , rust blocks unwaneted creation of copies
}
   //let r1 = &s; // no problem
    // let r2 = &s; // no problem
   let r3 = &mut s; // BIG PROBLEM
  let  r2 = &mut s;
}


fn change(stri: &mut String){
    stri.push_str(", wrold");
}


fn printstr(stri:String){
   println!("{stri}");
}

//slices types 
// str:-string slices
// [i32]:- intiger slice