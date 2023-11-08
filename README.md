# Rust Playground

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
