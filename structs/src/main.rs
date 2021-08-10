fn main() {
// Struct user:
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

//tuple Struct:
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    is_active: true,
    sign_in_count: 1,
   };
// Sin sintaxis de actualización:
   let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    is_active: user1.is_active,
    sign_in_count: user1.sign_in_count,
   };
// Con sintaxis de actualización:
let user3 = User {
    email: String::from("name@example.com"),
    username: String::from("Nice_username"),
    ..user2
};
println!("{}", user3.email);
fn _build_user(email: String, username: String) -> User{
    return User{
    email,
    username,
    is_active: true,
    sign_in_count: 8
    }
}

}

