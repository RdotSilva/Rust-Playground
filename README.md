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
