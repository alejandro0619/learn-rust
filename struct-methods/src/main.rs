//siguiendo con el ejemplo de Rectangle:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32
}
impl Rectangle{
    fn area(&self) -> u32 {
        return self.width * self.heigth;
    }
    fn can_hold(&self, other: &Rectangle) ->  bool {
        return self.width > other.width && self.heigth > other.heigth;
    }
//funciones asociadas:
fn square(size: u32) -> Rectangle{
    Rectangle {
        width: size,
        heigth: size
    }
}
}


fn main() {
    let rect = Rectangle{
        heigth: 30,
        width: 50
    };
    let rect1 = Rectangle{
        heigth: 10,
        width: 40
    };
    let rect2 = Rectangle{
        heigth: 5,
        width: 5
    };
    let square = Rectangle::square(3);
    println!("{:?}", square);

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
}
