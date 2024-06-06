struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    let x = 1;

    // matching literals
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }

    let x = Some(5);

    // matching literals in patterns
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Got some other value: {y}"),
        _ => println!("Got nothing"),
    }

    let x = 1;
    
    // matching multiple patterns
    match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
    }

    let x = 3; 

    // matching ranges
    match x {
        1..=5 => println!("one to five"),
        _ => println!("anything else"),
    }

    let x = 'c';
    
    // character ranges
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("other character"),    
    }

    // destructuring structs
    let p = Point { x: 0, y: 5 };

    let Point { x: a, y: b } = p;
    println!("a: {a}, b: {b}");

    // shorcut
    let Point { x, y } = p;
    println!("x: {x}, y: {y}");
    
    // matching structs
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // a complex, deep match with tuples and structs
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {feet}, inches: {inches}, x: {x}, y: {y}");

    // The following code works because the '_' never binds, and therefore
    // never takes ownership of s.
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point3D { x: 0, y: 0, z: 0 };
    
    match origin {
        // Ignores all other fields
        Point3D { x: 0, .. } => println!("x is {x}"),
        Point3D { x, y, z } => println!("x: {x}, y: {y}, z: {z}"),
    }

    let numbers = (1, 2, 3, 4, 5, 7, 9, 104);

    // expanding with tuples
    let (first, .., last) = numbers;
    println!("Some numbers: {first}, {last}");

    let num = Some(4);

    // match guards
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let msg = Message::Hello { id: 5 };

    // @ Bindings    
    match msg {
        Message::Hello {
            // tests if the id is in range 3..=7, and if so, binds it to id_variable
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}
