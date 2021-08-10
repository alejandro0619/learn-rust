use std::io;
#[derive(Debug)]
struct Rectangle {
 width: u32,
 heigth: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.heigth
  }
}

fn receive_input() -> (u32, u32){
    let mut _width_size = String::new();
    let mut _height_size =  String::new();
    println!("Enter width");
 
    io::stdin().read_line(&mut _width_size)
      .expect("Error at reading line");
 
    let mut _width_size: u32 = _width_size.trim().parse()
      .expect("Cannot parse letter, please aenter a number");
 
    println!("Enter a height");
 
    io::stdin().read_line(&mut _height_size)
      .expect("Error at reading line");
 
    let mut _height_size: u32 = _height_size.trim().parse()
      .expect("cannot parse letter, please enter a number");

    return (_width_size, _height_size);
}

fn main() {
    let (width_size, height_size) = receive_input();
     let rect = Rectangle {
         width: width_size,
         heigth: height_size
     };
     println!("{:#?}", rect);
     println!("The area of the rectangle is: {}",rect.area());
}

