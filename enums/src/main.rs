//enums:
enum IpAddr {
    _V4,
    V6
    }
struct IpAdrrStruct{
    _kind: IpAddr,
    _address: String
    }
// otra forma de hacerlo:
enum IpAddrkind{
    V4(u8, u8, u8, u8),
    _V6(String)
} 

enum Message {
    QUit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self){
        //method body
    }
}
//prgram starts:
fn main() {

let m = Message::Write(String::from("Hello"));
m.call();
let _loopback = IpAdrrStruct{
    _kind: IpAddr::V6,
    _address: String::from("::1")
};
let _home: IpAddrkind = IpAddrkind::V4(127,0,0,1);

}
