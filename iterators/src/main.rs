fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    let mut v1_iter = v1.iter();

    println!("0: {:?}", v1_iter.next());
    println!("1: {:?}", v1_iter.next());
    println!("2: {:?}", v1_iter.next());
    println!("3: {:?}", v1_iter.next());

    let total: i32 = v1_iter.sum();
    println!(".sum() takes ownership, i.e., consumes the iter: {total}");

    // we can do maps, filters, and reduces on iters  
}
