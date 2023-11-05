mod my_functs;

use crate::my_functs::add_five;

// Impport all
// use crate::my_functs::*;

// Import several
// use crate::my_functs::{add_five, subtract_10};

fn main() {
    // Everything is defaulted to immutable
    // Use mut to make data mutable
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    x = 70;
    println!("x is {}", x)
}
