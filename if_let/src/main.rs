enum Coin {
    Penny,
    _Nickel,
    _Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    Alabama,
    //and so on..
}

fn main() {
    let config_max = Some(3u8);
    // match on only one case (where a match would have a _ => ()
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(State::Alabama);
    check_quarter(coin2);
    check_quarter(coin1);
}

// if let with else
fn check_quarter(coin: Coin) {
    if let Coin::Quarter(state) = coin {
        println!("The quarter is from {state:?}");
    } else {
        println!("Not a quarter");
    }
}
