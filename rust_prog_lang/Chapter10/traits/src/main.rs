pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline,
                                  self.author,
                                  self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("notify: {}", item.summarize());
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x > y, x = {}", self.x);
        } else {
            println!("y > x, y = {}", self.y);
        }
    }
}

fn main() {
   let tweet = Tweet {
        username: String::from("borginer"),
        content: String::from("don't smoke kids"),
        reply: false,
        retweet: false,
    }; 
    
    println!("tweet summery\n{}", tweet.summarize());
    notify(&tweet);
}
