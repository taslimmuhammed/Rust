use std::fmt::{Display, Debug};

//shared behaviour accross multiple types

pub trait Summary{
     fn summarize(&self)->String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//returns any type the impliment Summary
pub fn returns_summarizable()-> impl Summary{
    Tweet{
        username:"Taslim Muhammed Moosa".to_string(),
        content:"Hi, Hello".to_string(),
        reply:false,
        retweet:false
    }
} 
fn some_functions<T,U>(t: &T, u:&U)->i32
   where T: Display +Clone,
         U:Clone+Debug
         {
                  1
         }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{}: {}", self.username, self.content)
    }
}



fn main() {
    
}
