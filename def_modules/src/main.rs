// Use Asparagus from the vegeteble module from the garden module.
// This is located in this crate.
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");    
}
