fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");

    //  tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {y}");
    let x = tup.0;
    println!("The value of x is: {x}");

    //arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for i in a {
        print!("{}, ", i);
    }
    println!();
    let a = ['h'; 5];
    for i in a  {
        print!("{}, ", i);
    }
    println!();
}
