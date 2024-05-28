struct Point<T> {
    x: T,
    y: T,
}

struct MismatchedPoint<T, U> {
    x: T,
    y: U,
}

// 'impl<T>' required to declare types we are implementing on.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// impl on just floats
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// One thing to note, is that most of the time, we only need to include '<>'in
// our struct/function/enum definition, and not when declaring or calling.
fn main() {
    let number_list = vec![34, 50, 25, 100, 32];
    let result = largest(&number_list);
    println!("The largest number is {result}");
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    // Declaring struct with generics. It automatically assums int based off
    // the first instance of 'T'.
    let point = Point { x: 5.6, y: 4.8};
    println!("x: {}, y: {}", point.x(), point.y);
    println!("Distance from origin: {}", point.distance_from_origin());

    let point = MismatchedPoint { x: "hey", y: 40.38 };
    println!("x: {}, y: {}", point.x, point.y);
}

/// Function which takes a list of any tipe and finds the largest element.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
