use crate::List::{Cons, Nil};

fn main() {
    // stores 5 on the heap.
    // Box<T> is a pointer.
    let b = Box::new(5);
    println!("b = {}", b);

    // Potentially infinite recursive list called cons list
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
