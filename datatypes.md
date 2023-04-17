# Data types:
>Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.

## Scalar types:
### integers

|    Length     |     Signed    |    Unsigned   |
| ------------- | ------------- | ------------- |
| 8-bit  | i8  | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit |	i128 | u128 |
| arch | isize | usize |

isize and usize are decided by the CPU architecture running the program. If a 32-bit CPU is running the program the length is 32-bit, wheres as a 64-bit CPU will have a length of 64-bit.

| Number literals | Example | 
| ------------- | ------------- |
| Decimal | 98_222 |
| Hex | 0xfe |
| Octal | 0o76 |
| Binary | 0b1111_1110 |
| Byte (u8 only) | b'A' |

>Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. 
 
>Relying on integer overflow’s wrapping behavior is considered an error.
### floating-point

### numbers

### Booleans

### characters


## compound types: