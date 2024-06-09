use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// DECLARATIVE MACROS: declarative paradigm
// Simplified vec! definition:
// Allows vec! to come into scope. In this case, it will come into scope when
// this crate comes into scope.
#[macro_export]
macro_rules! vec {
    // matches any number of args where each arg is an expression.
    ( $( $x:expr ),* ) => {
        {
            // Creates new Vec
            let mut temp_vec = Vec::new();
            $(
                // populates Vec
                temp_vec.push($x);
            // performs any number of times
            )*
            // Returns Vec
            temp_vec
        }
    };
}    

// PROCEDURAL MACROS: procedural paradigm
// See hello_macro_derive
// Must be in its own, specific type of crate.
// must input and output TokenStream

// DERIVE MACROS
// See hello_macro and hello_macro_derive crate
#[derive(HelloMacro)]
struct Pancakes;

// ATTRIBUTE-LIKE MACROS
/* Is similar to the derive macro.
   the attribute:
   #[route(GET, "/")]
   would be prrovided by the proc macro:
   #[proc_macro_attibute]
   pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
*/

// FUNCTION-LIKE MACROS
/* Take tokenStream param and manipulates it
   an example is the sql! macro.
*/

fn main() {
    let v = vec![1, 2, 3];
    /* Will be replaced by:
    let v = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec;
    }
    */
    v.iter().for_each(|x| print!("{} ", x));
    println!();

    Pancakes::hello_macro();
}
