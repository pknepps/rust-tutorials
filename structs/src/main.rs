/// A struct which represents a user.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// A tuple struct representing a color.
struct Color(i32, i32, i32);

/// A unit-like struct.
struct AlwaysEqual;

/// The entry point for the program.
fn main() {
    // Mut makes the entire struct mutable.
    let mut user1 = User {
        active: true,
        username: String::from("usernameexample1"),
        email: String::from("username@example.com"),
        sign_in_count: 1,
    };
    print_user(&user1);
    user1.email = String::from("changedEmail@example.com");
    println!("new email: {}", user1.email);
    
    let user2 = build_user_without_shorthand(
        String::from("person@ex.com"), String::from("person123"));
    print_user(&user2);

    let user2 = build_user(String::from("person@ex.com"), String::from("person123"));
    print_user(&user2);

    // Heres a shortcut for creating one struct from another.
    // This is called 'struct update syntax'.
    let user3 = User {
        email: String::from("third@ex.com"),
        ..user2
    };
    print_user(&user3);

    // Declaring a tuple struct.
    let brown = Color(255, 255, 127);
    println!("brown: r:{},g:{},b:{}", brown.0, brown.1, brown.2);

    // Declaring a unit-like struct.
    let _a = AlwaysEqual;

    // Now let's change the way we've been printing the User struct.
    // Lets use debug prints
    println!("Built in way to print debug: {:?}", user1);
    println!("Pretty print: {:#?}", user1);
    // debug macro, also prints line number and file.
    dbg!(&user1);
    // We can do this since User derives Debug.
}

/// Creates a new user from the givem email and username.
fn build_user_without_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        // Notice how the following lines repeat var names.
        username: username,
        email: email,   
        sign_in_count: 1,
    }
} 

/// Creates a new user from the given email and username with shorthands. 
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // shorthand
        username,
        email,   
        sign_in_count: 1,
    }
} 

/// Prints the given user
fn print_user(user: &User) {
    println!("user:\n\tactive: {},\n\tusername: {},\n\temail: {},\n\tsign_in_count: {},", 
        user.active, user.email, user.username, user.sign_in_count);
}
