fn main() {
    let number = 3;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = if number < 5 {number + 1} else {32};
    println!("number is {number}");

    // loops
    let mut cnt = 0;
    println!("The loop ran {} times", loop {
        cnt += 1;
        if cnt >= 10 {
            break cnt;
        }
    });

    //labeling loops
    let mut cnt = 0;
    'counting: loop {
        println!("count = {cnt}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // breaks inner loop
                break;
            }
            if cnt == 2 {
                // breaks outer loop
                break 'counting;
            }
            remaining -= 1;
        }
        cnt += 1;
    }
    println!("End count = {cnt}");

    // range
    for i in (1..4).rev() { // loops through the numbers 1-4 in reverse
        println!("{i}");
    }
}
