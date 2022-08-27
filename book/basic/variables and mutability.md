## Variables
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

It's important to get this compile-time errores when we attempt to change a value that's designated as inmutable because this very situation can lead us to bugs. If one part of the code operates under the assumption that a value will never change, and another part of the code changes that value arbitrary, it's possible to that the first part won't do what it was coded to do. The cause of this of bug can be difficult to track down after the fact,specially when the second pieace of the code only changes a value sometimes.

but mutability can be very useful, and can make code more convenient to write. Variables are inmutable only by default, We can make them mutable by adding the word "mut" when declaring.

```rust
fn main() {
let mut x = 5;
println!("The value of x is {}", x);

x = 6;
println!("The value of x is {]", x);
}
```

This will work out just fine.


## Constanst
Like inmutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

1. We are not allowed to use "mut" with constants. Constans aren't just inmutable by default - they are always inmutable. 
2. We can declare a constant using the "const" keyword instead of "let" keyword.
3. Constant's type must be annotated when declaring.
4. Constants may be set to constants expressions, no the result of a value that could only be computed at runtime.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3
```

The constant name is THREE_HOURS_IN_SECONDS and its value is set to the result of multiplying 60 (the number of seconds in a minute) by 60 (The number in minutes in an hour) by 3 (The number of hours we want to count in this program). 

## Shadowing

We can declare a variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second varible's value is what the program sees when the variable is used. We can shadow a variable by using the same variable's name and repeating the use of the "let" keyword.

```rust
fn main(){
let x = 5;
let x = x + 1;

{
	let x = x * 2;
	println!("x in the inner scope is: {}", x);
}
println!("x in the outer scope is: {}", x);
}
```

It first binds x to 5, then it shadows it with x + 1, which means, 5 + 1 so the variable will be now holding th evalue of 6. Then, create a scope, within it we shadows 6 with x * 2, which means, 6 * 2 = 12 which will be only available inside that scope, we print it, we get out of the inner scope and we print x (6) again... Yeah, kinda odd.

It is fundamentally different from changing a value using "mut", here we do not reassign a variable, we create a new variable using the old's variable name, but this will keep being inmutable. Even, we can change the variable type but using the same name, whether mutating a variable will let us change its value but not its type.

For example, say pur program asks a user to show how many spaces they want between some text by inputting spaces characters, and then we want to store that input as a number:

```rust
let mut spaces = "   ";
spaces = spaces.len();
```
The error says we are not allowed to mutate variable's type. So the best approach is to shadow the variable.

```rust
let spaces = "    ";
let spaces = spaces.len();
```

The first spaces variable is a string type and the second spaces variable is a number type. Shadowing thus spares us from having to come up with different names, such as "spaces_str" and "spaces_num"; instead, we can reuse the simpler spaces name.