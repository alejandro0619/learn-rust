use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("create file... there was a problem")
            },
            other_error => panic!("a problem opiniong the file"),
        }
    };
    read();
}
fn read() {
    use std::io;
    use std::io::Read;
    use std::fs::File;
    fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    }
    read_username_from_file();
}