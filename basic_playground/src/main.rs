const OUR_COURSE: &str = "Rust Playground";

fn main() {
    println!("Hello World {}", OUR_COURSE);

    // Stack
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // For loop
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i)
        } else {
            print!("{}, ", i)
        }
    }

    // Mutation
    let mut z: i32 = 5;
    print!("z was {} ", z);
    z = 10;
    println!("but is now {}", z);

    // Float
    let freezing_temp: f64 = -2.4;
    println!("freezing_temp is {}", freezing_temp);

    // Modulo
    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("is_zero_remainder is {}", is_zero_remainder);

    // Char (single quotes for chars)
    let my_char: char = 'z';
    println!("my_char is {}", my_char);

    let emoji_char: char = '😎';
    println!("emoji_char is {}", emoji_char);

    // Float array stored on the sack (fixed size)
    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is {:?}", my_floats);

    // Map over an array
    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("my_floats_new is {:?}", my_floats_new);

    // String literal stored on Stack
    let name: &str = "Ryan";
    println!("name is {:?}", name);

    // Dynamic string
    let dynamic_name: String = String::from("Ryan");
    println!("dynamic_name is {:?}", dynamic_name);
    println!("my dynamic_name stored in memory {:p}", &dynamic_name);

    // Reference to a string
    let str_slice: &str = &dynamic_name[0..5];
    println!("str_slice is {:?}", str_slice);

    // Vector of characters (basically a string)
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('.');
    println!("chars is {:?}", chars);
    dbg!(&chars); // Debug statement uses a reference

    // Returns an enum but we can use unwrap
    let removed_char: char = chars.pop().unwrap();
    println!("removed_char is {:?}", removed_char);
    println!("chars is {:?}", chars);
}
