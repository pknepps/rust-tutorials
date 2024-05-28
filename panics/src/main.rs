fn main() {
    panic!("This chapter is a bit different. Comment this panic to see the out-of-bounds error.");
    
    out_of_bounds();
}

/// Throws an out-of-bounds error with panic!.
fn out_of_bounds() {
    let v = vec![1, 2, 3];
    v[31];
}
