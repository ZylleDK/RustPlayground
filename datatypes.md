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

### floating-point

### numbers

### Booleans

### characters


## compound types: