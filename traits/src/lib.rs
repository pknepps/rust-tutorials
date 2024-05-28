// Declares a trait which has a summarize method we can call on all types that
// have the 'Summary' trait.
pub trait Summary {
    // A type with this trait must have a fn with this signature exactly.
    fn summarize(&self) -> String;

    // Default function which does not need to be impl.
    // Note: we can call other functions in this impl block even though 
    // they have not been defined.
    fn print_summary(&self) {
        println!("{}", self.summarize());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implements the summary trait. You must implement all non-default traits.
// Note: You can not impl methods not part of Summary in this block.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// Also Note: you can only implement external traits on types which exist local
// to this crate. In this case, NewsArticle, and Tweet are the only available 
// types we can implement external traits for in this file. However, we can 
// implement internal traits on external types, like the String example below.
// Finally, we cannot have general impl blocks or impl external traits for
// external types.

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

    // Overwrites the default function
    fn print_summary(&self) {
        println!("I overwrote the default method");
    }
}

impl Summary for String {
    fn summarize(&self) -> String {
        String::from("String has implemented summarize")
    }
}
