fn add_one(x: i32) -> i32 {
    x + 1
}

// Takes a function as an argument (higher order function)
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    // This enum varient is a function.
    Value(u32),
    _Stop,
}

// Returning closures
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // Using functions as arguments.
    println!("The answer is: {}", do_twice(add_one, 5));
    // Mapping with enum variants.
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    list_of_statuses.iter().for_each(|status| if let Status::Value(val) = status { 
        print!("{val} "); 
    });
    println!();
    // Catching returned closures.
    let closure = returns_closure();
    println!("5 + 1 = {}", closure(5));
}
