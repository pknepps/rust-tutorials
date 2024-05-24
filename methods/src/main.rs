#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associated functions. Methods have the parameter &self.
impl Rectangle {
    // &self is of type &Self
    // Self is alias for the type impl is for (in this case Rectangle).
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // we are allowed functions with the same name as fields
    fn width(&self) -> bool {
        self.width > 0
    }
    // An associated function which is not a method.
    /// Constructor for a square.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
 
// A second impl block.
impl Rectangle {
    /// Tests if the given rectangle can fit completely in this one.
    fn can_hold(&self, other: &Rectangle) -> bool {
       self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    if rect1.width() {
        println!("The rectangle has positive width.");
    }
    
    // The two lines are equivalent:
    _ = (&rect1).area();
    _ = rect1.area();
    // This is because rust automatically calls on reference if needed.
    
    // Using args in methods.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));    
    
    // Calling a non-method associated function.
    let square = Rectangle::square(3);
    println!("The square has side length: {}", square.width);
}
