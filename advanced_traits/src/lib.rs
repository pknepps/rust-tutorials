use std::ops::Add;
use std::fmt;

pub trait Iterator {
    // Placeholder type, which allows us to define the type per implementation.
    // Called an associated type.
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

// We can overload operators. In this case we overload '+'.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// The addition trait. 
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Overloading operators on 2 different types.
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

// Traits with same-named methods.
trait Pilot {
    fn fly(&self) -> &str;

    fn other_stuff() -> i32;
}

trait Wizard {
    fn fly(&self) -> &str;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> &str {
        "This is your captain speaking."
    }

    fn other_stuff() -> i32 {
        4
    }
}

impl Wizard for Human {
    fn fly(&self) -> &str {
        "Up!"
    }
}

impl Human {
    fn fly(&self) -> &str {
        "*waving arms furiously*"
    }

    fn other_stuff() -> i32 {
        17
    }
}

// Forcing a type to implement a needed trait if implemented.
// In this case, the fmt::Display trait is needed to have been implemented.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) -> String {
        let output = self.to_string();
        let len = output.len();
        format!("{}\n", "*".repeat(len + 4))
        + &format!("*{}*\n", " ".repeat(len + 2))
        + &format!("* {} *\n", output)
        + &format!("*{}*\n", " ".repeat(len + 2))
        + &format!("{}\n", "*".repeat(len + 4))
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next() {
        let mut x = Counter { count : 1 };
        if let Some(n) = x.next() {
            assert_eq!(n, 2);
        } else {
            panic!("get_next() failed");
        }
    }

    #[test]
    fn add_points() {
        let p1 = Point { x: 1, y: -4 };
        let p2 = Point { x: 4, y: 3 };
        assert_eq!(p1 + p2, Point { x: 5, y: -1 });
    }

    #[test]
    fn add_lengths() {
        let m = Meters(2);
        let mm = Millimeters(1243);
        // order matters, since m.add(mm) is not implemented.
        let Millimeters(x) = mm + m;
        assert_eq!(x, 3243);
    }

    #[test]
    fn same_named_stuff() {
        let person = Human;
        // defaults to the direct impl.
        assert_eq!(person.fly(), "*waving arms furiously*");
        // calling other implementations
        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(Wizard::fly(&person), "Up!");
        // calling associated functions: defaults to direct impl.
        assert_eq!(Human::other_stuff(), 17);
        // calling trait function
        assert_eq!(<Human as Pilot>::other_stuff(), 4);
    }

    #[test]
    fn force_trait() {
        let p = Point { x: 4, y: -1 };
        assert_eq!(p.outline_print(), "\
***********
*         *
* (4, -1) *
*         *
***********
");
    }
}
