fn main() {
    let s = String::from("hello");
    println!("s: {s}");
    let s2 = s;
    // We are not allowed to use s as it has already been dereferenced. It's contents were moved.
    println!("s2: {s2}");
    let s = s2.clone();
    println!("Now we can use both s ({s}) and s2 ({s2})!");

    let s = gives_ownership();
    takes_ownership(s);
    println!("We can no longer access s");
    let x = 5;
    makes_copy(x);
    println!("We can still access x ({x})");
}

/**
 * Since the string takes up memory on the heap. This function takes ownership of the string,
 * removing the rights of 'main' to access the parameter.
 * @param some_string: A string to take ownership from. The caller will no longer have ownership
 *                     after the call. 
 */
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("after giving s: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

/** 
 * Since integer is a scalar/primitive. This function simply copies the parameter, allowing
 * 'main' to still use it.
 * @param some_integer: An integer to copy.
 */
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("after giving x: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

/**
 * Creates a variable and passes ownership as a return.
 * @returns ownership of the created variable.
 */
fn gives_ownership() -> String {
    let s = String::from("Ownership of this string was passed from another function");
    s
}
