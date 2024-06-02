use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *a);
}

/// our own smart pointer.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
