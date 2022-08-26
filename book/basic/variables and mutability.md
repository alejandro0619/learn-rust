"This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers.  However it's still possible to make variable mutable." says the Offical book.

When a variable is inmutable, once a value is bound to a name, that means, the value can't be changed.

```rust
fn main (){
	let x = 5;
	println!("The value of x is {}", x);
	x = 6;
	println!("The value of x is {}", x);
}
```

And this will throw an error that indicates that the cause is "you cannot assign twice to inmutable variable."


