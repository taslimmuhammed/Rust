use std::{error, io};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::net::IpAddr;
fn main() {
    let f = File::open("hello.txt");
    
    let _f =match f{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
               Ok(fc)=>fc,
               Err(e)=>panic!("problem creating the file: {:?}",e)
            },
            other_error=>{
                panic!("problem opening file: {:?}",other_error)
            }
        }
    };
    useExcept();
    using_questionmark();
}

//same code as above usig unwrapping
fn unwrpping(){
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|e|{
                panic!("Problem creating file : {:?}",e)
            })
        }
        else {
            panic!("problem creating the file {:?}",error);
        }
    });
}



fn useExcept(){
    let ip :IpAddr = "128.0.0".parse().expect("Error parsing the value");//panic text when error happen
}

fn using_questionmark()-> Result<String,io::Error>{
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    //question mark worok just as match for Result enum
    //-if the signal is Ok then it goes to next statment
    //-if the signal is Err then it panics by the error
    Ok(s)
}