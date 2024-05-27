fn main() {
    // Creating vectors.
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    
    // Populating vectors
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    // Reading vectors
    // Will panic at index out-of-bounds.
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Handles index out-of-bounds.
    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("Ther is no third element."),
    }
    
    // Now that we've borrowed a reference to the 3rd element of v,
    // we can not add new elements as it would throw an error.    

    // Iterating
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // dereferenced i and adds 50 to it.
        *i += 50;
        println!("{i}");
    }

    // Using multiple types
    enum Type {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let v = vec![
        Type::Int(3),
        Type::Float(9.12),
        Type::Text(String::from("great"))
    ];
    
    for i in v {
        match i {
            Type::Int(x) => println!("{x}"),
            Type::Float(x) => println!("{x}"),
            Type::Text(x) => println!("{x}"),
        }
    }

    // removing elements
    let mut v = vec![1, 2, 3, 4, 5];
    if let Some(a) = v.pop() {
        println!("Removed {a}");
    }
    print!("Remaining elements: ");
    for i in &v {
        print!("{i}, ");
    }
    println!();
}
