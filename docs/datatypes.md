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
### floating-point numbers
Rust has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. 
### Booleans
As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool.
### characters
Rust’s char type is the language’s most primitive alphabetic type. Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.
Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust. 


## compound types:
### The Tuple type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
```
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
#### Accessing Tuppe elements
```
let five_hundred = tup.0
let six_point_fout = tup.1
```
### The Array type
Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
Arrays are useful when you want your data allocated on the stack rather than the heap

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
```
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
#### Accessing Array elements
```
let first = a[0];
let second = a[1];
```
### The Vector type
A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size

### Static string
A static string is a fixed sized string that is stored on the stack. A static string is defined like ex:
```
let s = "This is a static string";
```

### Dynamic strings
A dynamic string is a heap allocated string, that can change in size during runtime. A dynamic string is defined like ex:
```
let mutable_string = String::from("This is a dynamic string");
```

And can be changed during runtime in the following ways:
```
mutable_string.push_str(" in action!")
```

```
mutable_string = String::from("Clears old string with new string!");
```

### The Slice type
