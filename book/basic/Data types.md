Since Rust is a statically typed programming language, it must know the type of data we are using in order to work with it correctly, although most of the time, the compiler can infer the type by the value and how we use it. In cases when many types are possible, such as when we're converting a String to a number, we must specify the type manually:

```rust
let convert:u32 = "3".parse().expect("Not a number");
```

If we don't add the type annotation, the compiler will display an error.

As we said before, every value in Rust is of a certain data type, we'll look at two data types subsets: scalar types and compound types


### Scalar types
Represents a single value. Rust has four types of scalar types: Integers, floating-point numbers, booleans, and characters.

#### Integers
An integer is a number without a fractional component. We have both unsigned and signed integers as shown following:

**Lenght: 8-bit - i8 - u8
Lenght: 16-bit - i16 - u16
Lenght: 32-bit - i32 - u32
Lenght: 64-bit - i64 - u64
Lenght: 128-bit - i128 - u128
Lenght: arch - isize - usize**

Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the tablet as arch: 64 bits if you are on a 64-bit achitecture, the same with 32 bit architecture, using 32-bits.

We can write integer literals in the following way:

Decimal: 98_222
Hex: 0xff
Octal: 0o77
Binary: 0b1111_0000
Byte: b'A'

Number literals can also use _ as a visual separator to make the number easier to read, such as 98222 = 98_222.

Also, Rust allows to use a type suffix: such as 57u8 is an unsigned integers with the value 58.

Note: numbers by default are set to i32.

### Floating-point 
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust's floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it's roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.
