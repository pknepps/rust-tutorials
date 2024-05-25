enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // and so on..
}

impl Coin {
    // Uses the match expression to match the value of the enum.
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter fom {:?}!", state);
                25
            },
        }
    }
}

fn main() {
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter(UsState::Alabama);
    let q1 = Coin::Quarter(UsState::Alaska);
    println!("The value of a penny is: {}", p.value_in_cents());
    println!("The value of a nickel is: {}", n.value_in_cents());
    println!("The value of a dime is: {}", d.value_in_cents());
    println!("The value of a quarter is: {}", q.value_in_cents());
    let _ = q1.value_in_cents();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let x = match six {
        Some(x) => x,
        // must have both since matches cover both.
        None => 0,
    };
    let y = match none {
        Some(x) => x,
        None => 0,
    };
    println!("The value of six is: {x}");
    println!("The value of none is {y:?}");
    println!("A roll of 7 is {}", dice_roll(7));
    println!("A roll of 3 is {}", dice_roll(3));
    println!("A roll of 10 is {}", dice_roll(10));
}

// How to pattern match Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // catching as a variable
        Some(i) => Some(i + 1),
    }
}

// Catch-all patterns.
fn dice_roll(x: u8) -> String {
    match x {
        7 => String::from("Robber"),
        3 => String::from("wheat"),
        // we can use a variable. In this case, we are throwing away the other cases.
        _ => String::from("invalid"),
    }
}
