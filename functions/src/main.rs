fn main() {
    println!("Hello, world!");

    another_function(5);

    /* expressions and statements
       expressions yield return values, statements do not.
       in the below example, the 'let' lines are statements.
       but the lines wrapped in curly braces are an expression,
       and so is the 'y + 1 line. */
    let x = {
        let y = 3;
        // expressions do not end in semicolons.
        y + 1
    };
    
    println!("The value of x is: {x}");
    println!("One more than x is: {}", plus_one(x));
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

/**
 * This function has a return value.
 * Note: there is no return statement, because the last (and only line) is an expression.
 */
fn plus_one(x: i32) -> i32 {
    x + 1
}
