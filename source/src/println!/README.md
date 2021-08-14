# 📍 Println!

Printing to console is handled by a serie of macros, lets take a look into it:

- format!: write formatted text to String
- print!: same as format! but the text is printed to the console (io::stdout).
- println!: same as print! but a newline is appended.
- eprint!: same as format! but the text is printed to the standard error (io::stderr).
- eprintln!: same as eprint!but a newline is appended.

(Rust checks formatting correctness at compile time)

``` std::fmt ``` contains many traits which govern the display of text. The base form of two important ones are listed below:

``` fmt::Debug``` : Uses the ``` {:?} ``` marker. Format text for debugging purposes.
fmt::Display: Uses the ``` {} ``` marker. Format text in a more elegant, user friendly fashion.
Here, we used fmt::Display because the std library provides implementations for these types. To print text for custom types, more steps are required.