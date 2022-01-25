// Defining a enum
// Let's suppose we have a program that can only receive two types of IP addresses: IP version 4 and IP version 6.
// These are the only possibilities for an IP address that our program will come across: 
//we can enumerate all possible variants, which is where enumeration gets its name.
#[derive(Debug)]
enum IPAddrrKind {
  V4,
  V6
}
// struct that implements IPAdrrKind
#[derive(Debug)]
struct IP {
  version: IPAddrrKind,
  address: String
}
// we can create 2 instances:
// let v4 = IPAddrrKind::V4;
// let v6 = IPAddrrKind::v6;

impl IP {
  // we can receive a param and match the variant in use:
  fn from(ip_kind: IPAddrrKind, address: String) -> Self {
    match ip_kind {
    IPAddrrKind::V4 => IP { version: ip_kind, address },
    IPAddrrKind::V6 => IP { version: ip_kind, address },
    }
  }
  fn display(&self) {
    println!("Your IP is {} and the kind of {:?}", self.address, self.version);
  }
}

// ----- A BETTER WAY TO DO THIS WITHOUT ATTACHING A STRUCT:
// we can put data directly into each variant of the enum
// let's define a new enum:
#[derive(Debug)]
enum IPAddrrKindV2 {
  V4(String), // This variant receives a String
  V6(String), // this too.
}

// we can create 2 instances:
// let v4 = IPAdrrKinfV2::V4(String::from("127.0.0.1"));
// let v6 = IPAdrrKinfV2::V6(String::from("::1"));

impl IPAddrrKindV2 {
  fn from(kind: &str, address: String) -> Self {
    match kind {
      "V4" => IPAddrrKindV2::V4(address),
      "V6" => IPAddrrKindV2::V6(address),
      _ => panic!("Invalid version"),
    }

    
  }

  fn display(&self){
    match self {
      IPAddrrKindV2::V4(val) => println!(" Your IP is {} and the kind of V4", val), // prints out the value of the variant
      IPAddrrKindV2::V6(val) => println!(" Your IP is {} and the kind of V6", val), // prints out the value of the variant
    }
  }
}
pub fn start(){
  let v4 = IPAddrrKindV2::from(&String::from("V4"), String::from("127.0.0.1"));
  let v6 = IPAddrrKindV2::from(&String::from("V6"), String::from("::1"));
  v4.display();
  v6.display();
}