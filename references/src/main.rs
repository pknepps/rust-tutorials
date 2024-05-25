fn main() {
    let s1 = String::from("hello");
    // The '&' in the following line at '&s1' is a reference to s1.
    // instead of passing ownership, the variable is borrowed.
    let len = calculate_length(&s1);
    // &s1 and s1 are interchangeable here. As &s1 uses the value at that reference
    println!("The length of '{}' is {}.", &s1, len);

    // mutable references
    let mut s = s1;
    // Here we pass in a mutable reference. This allows us to modify s from the function change.
    // Note: we can only have 1 mutable reference of s, and no other references.
    //       or we can have any number of immutable references, and no mut ref.
    change(&mut s);
    println!("s has been changed to '{s}'");
}

/**
 * This function borrows s and returns it's length.
 * @param s: The string to borrow. Since s is a reference, s will not be dropped when it loses scope.
 * @return The length of s.
 */
fn calculate_length(s: &String) -> usize {
    // We are borrowing s. So we cannot modify it.
    s.len()
}

/**
 * This function borros s and changes it by appending to the end of it.
 * @param s: a mutable reference to s. Which will allow us to change s.
 */
fn change(s: &mut String) {
    s.push_str(", world");
}
