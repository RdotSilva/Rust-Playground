#[allow(dead_code, unused_variables)]
fn example_0() {
    let r: &i32; // 'a

    let x: i32 = 5; // 'b
    r = &x;

    println!("r: {}", r);
}

#[allow(dead_code, unused_variables)]
fn example_1() {
    // Allocate space in memory
    let highest_age: i32;

    // Initialize vars
    let alice_age: i32 = 20; // 'a
    let bob_age: i32 = 21; // 'b: 'a

    // Call function
    highest_age = largest(&alice_age, &bob_age);

    // Print output
    println!("Highest age is {}", highest_age);

    // This returns an i32 so we have to de-reference compare_1 and compare_2 when returning
    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_2() {
    // Allocate space in memory
    let highest_age: &i32;
    let new_value: i32;

    // Initialize vars
    let alice_age: i32 = 20; // 'a

    {
        let bob_age: i32 = 21; // 'b

        // Call function
        highest_age = largest(&alice_age, &bob_age);

        // Create a new value with a dereferenced value
        new_value = *highest_age;
    } // 'b out of scope

    // Print output
    println!("New value age is {}", new_value);

    // This example uses a lifetime of 'a
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_3_generics() {
    // Allocate space in memory
    let highest_age: &i32;
    let new_value: i32;

    // Initialize vars
    let alice_age: i32 = 20; // 'a

    {
        let bob_age: i32 = 21; // 'b

        // Call function
        highest_age = largest::<i32>(&alice_age, &bob_age);
        new_value = *highest_age;
    } // 'b out of scope

    // Print output
    println!("New value age is {}", new_value);

    // This uses generics T
    fn largest<'a, 'b: 'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'b T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
struct Person<'p, 'q> {
    name: &'p str,
    points: &'q f32,
}
