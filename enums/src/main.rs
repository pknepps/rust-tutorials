// Defines an enum.
enum IpAddrKind {
    V4,
    V6,
} 

// An enum variant.
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//a varied enum
enum Message {
    _Quit,
    _Move { x: i32, y: i32},
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

// we can implement methods on enums.
impl Message {
    fn _call(&self) {
        
    }
}

fn main() {
    // Creating instances of an enum.
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    let _home = IpAddr::V4(127, 0 ,0 , 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    // Using the Option enum. No need for Option:: since it is so common.
    let _some_number = Some(5);
    let _some_char = Some('l');
    let _absent_int : Option<i32> = None;
    println!("This project has no output, see the match project for uses of enums");
}
