// mod my_functs;
// mod other_functs;

// use crate::my_functs::add_five;
// use crate::other_functs::minus_functs::subtract_10;

// Impport all
// use crate::my_functs::*;

// Import several
// use crate::my_functs::{add_five, subtract_10};

// fn main() {
//     // Everything is defaulted to immutable
//     // Use mut to make data mutable
//     let mut x: u32 = 50;
//     println!("x is {}", x);

//     let y: u32 = add_five(x);
//     println!("y is {}", y);

//     let z: u32 = subtract_10(x);
//     println!("z is {}", z);

//     x = 70;
//     println!("x is {}", x)
// }

// Stack vs Heap
const MY_INTEGER: u8 = 10;
fn main() {
    // Stack
    let x: u8 = 50;
    println!("x is {}", x);

    // Heap
    let mut arr: Vec<u8> = vec![1, 2, 3, 4, 5];
    arr.push(10);
    println!("vec is {:?}", arr);

    println!("MY_INTEGER is {:?}", MY_INTEGER);
}
