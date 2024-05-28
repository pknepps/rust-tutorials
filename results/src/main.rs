use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};
use std::error::Error;

// One of the possible return values for main is Result<(), Box<dyn Error>>.
fn main() -> Result<(), Box<dyn Error>> {
    // File::open returns Result<std::fs:File, std::io::Error>
    // hello.txt does not exist, so file_result will be Err(error);
    let file_result = File::open("hello.txt");
    
    // Triple nested matches get a little busy. Closures are a good way to 
    // clean it up.
    let _file = match file_result {
       Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating file: {error:?}"),
            },
            other => panic!("Problem opening the file: {other:?}"),
        },
    };

    // Comment the match statements above to show how the following throw errors.
    
    // The .unwrap() method build into Result<T, E> will 'unwrap' Ok(val),
    // returning val, or panic in the case of an error. 
    let _file = File::open("hello.txt").unwrap();

    // The .expect() method is an .unwrap() which allows for custom error
    // messages. These messages should always describe what you expect to
    // happen, not what went wrong.
    let _file = File::open("hello.txt")
        .expect("hello.txt should be included in this project.");

    println!("Username found: {}", read_username_from_file()?);
    
    Ok(())
}


// Propagating, i.e. throwing errors up the stack.
fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        // If there is an error, return it so the caller can handle it instead.
        Err(error) => return Err(error),
    };
    
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        // If there is an error, return it so the caller can handle it instead.
        Err(error) => Err(error),
    }
}

// Propagating using the '?' operator. (Does the same as above, just better).
// Note: we can only use '?' in functions which have a return value of 
// Result<T, E> or Option<T> where ? early returns None.
fn _read_username_from_file_with_question() -> Result<String, io::Error> {
    // '?' propagates if there is an error.
    let mut username_file = File::open("hello.txt")?;
    // '?' also calls 'from()' which converts the Err(e) from File::open to 
    // the Err(e) specified in the return type of this function.
    let mut username = String::new();
    // propagates if there is an error.
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Final version of function simpified.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    // We can one-line everything
    File::open("hello.txt")?.read_to_string(&mut username)?;
    
    Ok(username)
}

// Even simpler version
fn _read_username_cheating() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
