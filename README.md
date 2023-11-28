# Rust Playground

## Basic Playground

Located in the `basic_playground/src` folder. Each section is separated into it's own file:

1. Enums
2. Structs
3. Traits
4. Polymorphism
5. Lifetimes
6. Patterns
7. Async calls
8. Collections

## Rust Integer Types

Rust provides a variety of integer types with different sizes and signedness. Here is a list of all the integer types, their byte size, and their minimum and maximum values:

### Signed Integers

| Type  | Size (Bytes) | Minimum Value                                        | Maximum Value                                       |
| ----- | ------------ | ---------------------------------------------------- | --------------------------------------------------- |
| i8    | 1            | -128                                                 | 127                                                 |
| i16   | 2            | -32,768                                              | 32,767                                              |
| i32   | 4            | -2,147,483,648                                       | 2,147,483,647                                       |
| i64   | 8            | -9,223,372,036,854,775,808                           | 9,223,372,036,854,775,807                           |
| i128  | 16           | -170,141,183,460,469,231,731,687,303,715,884,105,728 | 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| isize | \*           | _depends on the machine_                             | _depends on the machine_                            |

### Unsigned Integers

| Type  | Size (Bytes) | Minimum Value | Maximum Value                                       |
| ----- | ------------ | ------------- | --------------------------------------------------- |
| u8    | 1            | 0             | 255                                                 |
| u16   | 2            | 0             | 65,535                                              |
| u32   | 4            | 0             | 4,294,967,295                                       |
| u64   | 8            | 0             | 18,446,744,073,709,551,615                          |
| u128  | 16           | 0             | 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| usize | \*           | 0             | _depends on the machine_                            |

The `isize` and `usize` types are pointer-sized and can vary in size depending on the target architecture: 32 bits on x86 systems and 64 bits on x64 systems.

Integer literals can be written as decimal numbers, hexadecimal numbers (using an `0x` prefix), octal numbers (using an `0o` prefix), binary numbers (using an `0b` prefix), and byte (`u8` only) literals using a `b` prefix.

Example of integer literals:

```rust
let decimal = 98_222; // Decimal
let hex = 0xff; // Hexadecimal
let octal = 0o77; // Octal
let binary = 0b1111_0000; // Binary
let byte = b'A'; // Byte (u8 only)
```

## Data Type Cheat Sheet

Rust has several data types divided into different categories. Below is a table covering most of them:

| Category       | Data Type | Description                                               | Copy Trait  | Memory Allocation    |
| -------------- | --------- | --------------------------------------------------------- | ----------- | -------------------- |
| Integer        | i8        | 8-bit signed integer                                      | Yes         | Stack                |
| Integer        | i16       | 16-bit signed integer                                     | Yes         | Stack                |
| Integer        | i32       | 32-bit signed integer                                     | Yes         | Stack                |
| Integer        | i64       | 64-bit signed integer                                     | Yes         | Stack                |
| Integer        | i128      | 128-bit signed integer                                    | Yes         | Stack                |
| Integer        | isize     | Pointer-sized signed integer                              | Yes         | Stack                |
| Integer        | u8        | 8-bit unsigned integer                                    | Yes         | Stack                |
| Integer        | u16       | 16-bit unsigned integer                                   | Yes         | Stack                |
| Integer        | u32       | 32-bit unsigned integer                                   | Yes         | Stack                |
| Integer        | u64       | 64-bit unsigned integer                                   | Yes         | Stack                |
| Integer        | u128      | 128-bit unsigned integer                                  | Yes         | Stack                |
| Integer        | usize     | Pointer-sized unsigned integer                            | Yes         | Stack                |
| Floating-Point | f32       | 32-bit floating-point number                              | Yes         | Stack                |
| Floating-Point | f64       | 64-bit floating-point number                              | Yes         | Stack                |
| Boolean        | bool      | Boolean type (true or false)                              | Yes         | Stack                |
| Character      | char      | Single Unicode scalar value                               | Yes         | Stack                |
| Compound       | tuples    | Fixed-size collection of values, possibly different types | Conditional | Stack                |
| Compound       | arrays    | Fixed-size same-type collection of values                 | Conditional | Stack                |
| Textual        | &str      | String slice (view into a string)                         | No          | Stack (pointer data) |
| Textual        | String    | Grow-able UTF-8 string                                    | No          | Heap                 |
| Structs        | struct    | Custom data types                                         | Conditional | Stack or Heap        |
| Enums          | enum      | Type with a fixed set of variants                         | Conditional | Stack or Heap        |
| Pointer        | &T        | Reference pointer to type T                               | No          | Stack                |
| Pointer        | &mut T    | Mutable reference pointer to type T                       | No          | Stack                |
| Pointer        | Box<T>    | Heap-allocated type T                                     | No          | Heap                 |
| Function       | fn        | Function pointers                                         | Yes         | Stack                |
| Slice          | [T]       | Dynamically-sized view into a contiguous sequence, T      | No          | Stack (pointer data) |
| Trait Object   | dyn Trait | Dynamically dispatched trait object                       | No          | Heap (generally)     |
| Raw Pointer    | \*const T | Immutable raw pointer (unsafe)                            | Yes         | Stack                |
| Raw Pointer    | \*mut T   | Mutable raw pointer (unsafe)                              | Yes         | Stack                |
| Fixed-size     | array     | Fixed-size list of elements of the same type              | Conditional | Stack                |
| Variable-size  | vector    | Dynamically-sized, heap-allocated data structure          | No          | Heap                 |

Note: The 'Copy Trait' column specifies whether the data type implements the `Copy` trait by default. Some types, like structs, tuples, and arrays, may implement `Copy` conditionally, depending on whether their elements do.

Memory allocation for compound types such as structs and enums can be more complex depending on their definition and use case. Typically, they are placed on the stack, but if they include a type that requires heap allocation, like `String` or `Box<T>`, the data they own may be stored on the heap while the data structure itself is on the stack.

## Memory in Rust: Stack vs Heap

Understanding how Rust manages memory is crucial for writing efficient and safe code. Here's a quick cheat sheet on the differences between stack and heap memory in Rust:

### Stack

- **Allocation**: Automatic, handled at compile time.
- **Deallocation**: Automatic, occurs when the variable goes out of scope.
- **Access Speed**: Fast access due to fixed size and LIFO (last in, first out) order.
- **Size Limitation**: Limited to a fixed size; can lead to stack overflow if the limit is exceeded.
- **Use Cases**: Used for static memory allocation, which includes most primitives, fixed-size arrays, and structs that don’t contain `Box`, `Vec`, `String`, or other heap-based data types.
- **Lifespan**: Variables on the stack must have a known, fixed size at compile time.

\```rust
fn use_stack_memory() {
let x = 10; // stored on the stack
let y = "static string"; // stored on the stack
// 'x' and 'y' are popped off the stack when this function returns
}
\```

### Heap

- **Allocation**: Dynamic, managed at runtime.
- **Deallocation**: Manual or via Rust's ownership system, which uses RAII (Resource Acquisition Is Initialization) pattern.
- **Access Speed**: Slower than stack, due to dynamic memory management overhead.
- **Size Limitation**: Limited by the system’s available memory; more flexible, suitable for large or resizable data.
- **Use Cases**: Used for dynamic memory allocation, such as when the size of data is not known at compile time or when owning data needs to be shared or live longer than the scope it was created in.
- **Lifespan**: The heap allows for allocating memory that can live beyond the scope it was created in, transferring ownership as needed.

\```rust
fn use_heap_memory() {
let vec = Box::new(10); // Heap allocation
// 'vec' is deallocated when it goes out of scope, unless ownership is transferred
}
\```

### Comparing Stack and Heap

- **Performance**: Stack allocation/deallocation is faster than heap allocation/deallocation.
- **Flexibility**: Heap is more flexible, allowing for dynamic resizing and lifetime decoupling.
- **Memory Safety**: Rust enforces memory safety through its ownership rules, ensuring no dangling pointers, double frees, or memory leaks.
- **Variable Size**: Stack variables must have a fixed size, whereas heap variables can have dynamic size.
- **Scope**: Stack variables are tied to the scope they are defined in, whereas heap variables can be moved across scopes and threads.

Rust’s memory safety guarantees and performance benefits are largely due to its use of stack allocation wherever possible, and its careful management of heap-allocated data. It makes trade-offs between the stack and heap to optimize for both safety and speed.

# Ownership and Borrowing

Rust's ownership system is a set of rules that manages how the Rust compiler allocates and deallocates memory. It also ensures memory safety and concurrency safety. Here's how ownership and borrowing work in Rust, along with some examples:

## Ownership

In Rust, all values have a single owner. The ownership rules are as follows:

- Each value in Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

Here's an example of ownership:

```rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the string "hello"
    let s2 = s1; // s1 is moved into s2, s1 is no longer valid here
    println!("{}, world!", s2); // This works fine as s2 is the owner of the string
    // println!("{}, world!", s1); // This would fail as s1's value has been moved to s2
}
```

## Borrowing

Borrowing refers to passing references to a value without transferring ownership. There are two types of borrowing:

- **Immutable References**: Allows you to read data without changing it.
- **Mutable References**: Allows you to not only read but also modify data.

### Immutable References

You can create an immutable reference using the `&` syntax. It allows you to read data without taking ownership:

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
```

### Mutable References

You can create a mutable reference using the `&mut` syntax, which allows you to change the data the reference points to:

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed string: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

Note that you can have only one mutable reference to a particular piece of data in a particular scope. This restriction allows for mutation but in a very controlled fashion.

## Dereferencing

Dereferencing uses the `*` operator to access the value pointed to by a reference:

```rust
fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Dereference y to change the value of x
    }
    println!("x is now: {}", x);
}
```

This code shows how to modify `x` by using a mutable reference `y`. The line `*y += 1` changes the value that `y` points to, hence `x` is changed. When `y` goes out of scope, we can use `x` again, which now has the new value.

By understanding and using ownership and borrowing correctly, you can write safe and efficient Rust programs without needing a garbage collector.
