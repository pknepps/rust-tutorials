// Type alias. Kilometers is just i32.
type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 12;
    println!("x + y = {}", x + y);

    bar();
}

// Never or empty type. ! means the function never returns.
// It will only never return if the program ends here.
fn bar() -> ! {
    panic!("This function doesn't return anything");
}
