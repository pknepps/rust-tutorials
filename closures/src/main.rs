use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn capturing_references() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let borrows_via_closure = || println!("Print list from closure: {list:?}");
    
    println!("Before calling closure: {list:?}");
    borrows_via_closure();
    println!("After calling closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling mut closure");

    thread::spawn(move || println!("I have taken control of the list: {list:?}"))
        .join()
        .unwrap();
    // We now cannot use list
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // The following are equivalent
    fn not_closure (x: u32) -> u32 { x + 1 }
    let closure_with_annotations = |x: u32| -> u32 { x + 1 };
    // like vecs, the following functions types are determined on first use.
    // and cannot be changed.
    let closure_with_block = |x| { x + 1 };
    let closure_no_block = |x| x + 1;

    println!("{} {} {} {}", 
        not_closure(3), 
        closure_with_annotations(3),
        closure_with_block(3),
        closure_no_block(3)
    );
    
    capturing_references();
}
