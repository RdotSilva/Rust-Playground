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

const MSG_CONST: &str = "Hello World Constant";

fn main() {
    // Stack
    let x: u8 = 50;
    println!("x is {}", x);

    // Heap
    let mut arr: Vec<u8> = vec![1, 2, 3, 4, 5];
    arr.push(10);
    println!("vec is {:?}", arr);

    // A reference on the Stack pointing to a value
    let arr_2 = &arr[0..3];
    println!("arr_2 {:?}", arr_2);

    // Heap
    let mut s: String = String::from("Test String");
    s.push(' ');
    s.push('!');
    println!("s is {:?}", s);

    // A reference on the Stack pointing to a value on the Heap
    let s_2 = &s[0..5];
    println!("s_2 is {:?}", s_2);

    println!("MY_INTEGER is {:?}", MY_INTEGER);

    // Heap dynamic in size
    let st: String = String::from("Hello World");
    let st_2: &str = &st[0..5];
    println!("{}", st_2);

    // Stack and a reference to a string literal (that is stored statically)
    let msg: &str = "Hello World 2";
    println!("{}", msg);

    // Heap dynamic in size
    let msg_string: String = "Hello World 3".to_string();
    println!("{}", msg_string);

    println!("{}", MSG_CONST);
}
