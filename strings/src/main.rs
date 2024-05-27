fn main() {
    // Creating
    let mut s = String::new();
    
    let data = "initial contents";
    
    // All of the following work
    s = data.to_string();

    s = "initial contents".to_string();
    
    s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");

    // Updating strings
    println!("{s}");
    s = s + " here";
    println!("{s}");
    let h = " might".to_string();
    let st = s;
    // So that we can still reference h, st is moved and can no longer be accessed.
    s = st + &h;
    // note: + is alias for s.add(&str);
    println!("{s}");
    s = format!("The {s} be replaced later");
    println!("{s}");

    // Appending strings
    let mut s = String::from("foo");
    println!("{s}");
    s.push_str("bar");
    println!("{s}");
    // single char push
    s.push('.');
    println!("{s}");
    
    // Because strings do not encode bytes consistently, we cannot index them directly.
    // There are of course a few ways around this.
    for c in "3д".chars() {
        println!("{c}");
    }
    
    for b in "3д".bytes() {
        println!("{b}");
    }
    // We see that even though there are 2 chars, there are 4 bytes.

    // str.chars() returns an iterator.
    if let Some(c) = "Hello, World!".to_string().chars().next() {
        println!("{c}");
    }
}
