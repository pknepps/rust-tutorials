// The lifetime param 'a specifies that x and y (in the above cases, string1
// and string2) live the shorter of the two of their lifetimes. Since we don't
// know which one will be returned, this is needed.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//         ^       ^           ^           ^
// The same liftime param 'a.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Specifies that the struct lives as long as the string it is given.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Implementing on a struct with lifetimes is the following.
impl<'a> ImportantExcerpt<'a> {
    // We do not need to add lifetimes here because of the 3 rules.
    fn _level(&self) -> i32 {
        3
    }
}

// Static references live for the entire program.
fn return_static() -> &'static str {
    return "I lived for the entire program";
}

fn main() {
    let s = return_static();

    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
    let string1 = String::from("long string");
    {
        let string2 = "xyz";
        
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    } // we can no longer access result. As result only lives as long as the 
      // shorter of string1 and string2, which is string2's lifetime.

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not fund a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
    println!("{s}");
}
