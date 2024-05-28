use traits::{Summary, Tweet};
use std::fmt::Display;

fn main() {
    let tweet = returns_summarizable();
    println!("1 new tweet: {}", tweet.summarize());

    let s = String::new();
    s.print_summary();
    tweet.print_summary();
    
    notify(&s);
    other_notify(&tweet);
    third_notify(&s);
    fourth_notify(&s);
}

// Use of traits as params. We can take any type which implements Summary.
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
fn other_notify<T: Summary>(item: &T) {
    notify(item);
}

// Requires item to impl multiple traits. Called trait bounds.
fn third_notify(item: &(impl Summary + Display)) {
    notify(item);
}

// The above works in both styles.
fn _notify<T: Summary + Display>(_item: &T) {}

// Where clause is another way to do the same thing.
fn fourth_notify<T>(item: &T)
where
    T: Display + Summary,
{
    notify(item);
}

// We can also return a value of some type which impl a trait.
// Note: we cannot use this to return different types. Since we return a Tweet
// here, the only value allowed to be returned is a Tweet, i.e. we can not also
// return a NewsArticle in the same function.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
