// Let's make a Rectangle struct:
#[derive(Debug)]
struct Rectangle {
  w: i32, // width
  h: i32 // height
}

// Let's do a Rectangle struct method:
impl Rectangle {
  // calculate area:
  fn area(&self) -> i32 {
      self.w * self.h
  }
  fn display(&self)  {
    println!("{:?}", &self);
  }
}
pub fn start(){
  let rect1 = Rectangle {
    w: 5,
    h : 99
  };
  rect1.display();
  rect1.area();
}