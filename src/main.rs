fn add_five(num: u32) -> u32 {
    num + 5
}

// Everything is defaulted to immutable
fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    x = 70;
    println!("x is {}", x)
}
