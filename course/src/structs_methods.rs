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

  fn can_hold(&self, other_rect: &Rectangle) -> bool {
    self.w > other_rect.w && self.h > other_rect.h
  }
  // Associated functions:
  // Associated functions are methods of the struct
  // But we can create associated functions that doesn't have &self as the first parameter
  // And thus they are not methods because it does not need the instance of the struct
  // We already used one called String::from() which doesn't require to be called from an instance but from the type itself
  fn square(size: i32) -> Rectangle {
    Rectangle {
      w: size,
      h: size
    }
  }
}



pub fn start(){
  let rect1 = Rectangle {
    w: 5,
    h : 99
  };
  rect1.display();
  rect1.area();
  let can_hold: bool = rect1.can_hold( &Rectangle{ w: 5, h:3 } );
  // call the associated function withput &self param
  let rect2 = Rectangle::square(21);
}