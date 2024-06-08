// Declaring a static variable.
static mut COUNTER: u32 = 0;

// Extern allows the calling of code from another language.
// In this case, we are using the 'abs()' function from the C language.
extern "C" {
    // abs from the C standard lib.
    fn abs(input:i32) -> i32;
}

// Declaring an unsafe trait.
unsafe trait Foo {
    fn some_stuff(&self);
}

// The 5 things you can do in unsafe
fn main() {

    // 1) Dereferenceing a raw pointer
    let mut num = 5;
    // Raw pointers can ignore borrowing rules
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // We cannot be certain this address is in program memory.
    let address = 0x012345usize;
    let r = address as *const i32;
    // We need unsafe to dereference the raw pointers.
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
        // Will probably cause a seg-fault.
        // println!("r: {}", *r);
    }

    // 2) Calling an unsafe function/method.
    unsafe {
        dangerous();
    }

    // 2.1) Using extern functions
    unsafe {
        println!("Absolute value of -4 according to the C language: {}", abs(-3));
    }

    // 3) Accessing/modifying mutable static variables.
    unsafe {
        COUNTER += 3;
    }
    // Second unsafe block not needed, but used for demonstration.
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    5i32.some_stuff();

}

// 4) Implementing an unsafe trait.
unsafe impl Foo for i32 {
    fn some_stuff(&self) {
        println!("This is also unsafe: {self}");
    }
}

// Declaring an unsafe function.
// Note: normally we do not want to do this over making a unsafe block within a
// safe function.
unsafe fn dangerous() {
    let mut num = 4;
    let r = &mut num as *mut i32;
    // because this function is unsafe, it is treated as an unsafe block.
    *r = 8;
    println!("something dangerous (OoOo scary), {}", num);
}

// 5 is not seen here, but it is accessing the fields of a union.
