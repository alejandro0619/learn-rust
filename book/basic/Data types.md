Since Rust is a statically typed programming language, it must know the type of data we are using in order to work with it correctly, although most of the time, the compiler can infer the type by the value and how we use it. In cases when many types are possible, such as when we're converting a String to a number, we must specify the type manually:

```rust
let convert:u32 = "3".parse().expect("Not a number");
```

If we don't add the type annotation, the compiler will display an error.

As we said before, every value in Rust is of a certain data type, we'll look at two data types subsets: scalar types and compound types
