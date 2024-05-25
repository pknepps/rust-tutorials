// Slices are a reference to a contiguous subset of a collection.
fn main() {
    let mut s = String::from("Hello, World!");
    let first = first_word_no_slice(&s);
    println!("The first word (withot slices) has a length of: {}", first);
    // if we clear s, first still has reference that is unneeded and even harmful.
    s.clear();
    println!("The first word (withot slices) has a length of: {}", first);

    // let's look at some slices
    let s = String::from("Hello, World!");
    // creates a pointer with length 5 to index 0 of string s
    // can also use '..5' instead.
    let hello = &s[0..5];
    // creates a pointer with length 5 to index 7 of string s
    // if going to the end of a collection, can use 'n..'.
    let world = &s[7..12];
    println!("First word: {hello}");
    println!("Second word: {world}");

    // lets try the first funtion again
    let first = first_word(&s);
    println!("The first word is: {}", first);
    // now if we clear s, rustc will throw an error since the first word no longer has reference.

    // string literals (default type of string)
    let s: &str = "Hello, World";
    // s is a slice pointing to the specific binary.
    println!("{}", s);

    // a type of &str can be given a String reference.
    let s: &str = &String::from(s);
    println!("{}", s);
    // however, &String cannot be given &str

    // some examples:
    let my_string: String = String::from(s);
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("{}", word);
    let word = first_word(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "Hello, World!";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}", word);
}

/// Retreives the length of first word of the given string..
fn first_word_no_slice(s: &String) -> usize {
    // This function has a lot.
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

   s.len()
}

/// Retreives the first word of the string.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
