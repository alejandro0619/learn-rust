// Like tuples, structs can be made with different types
// Each field in the tuple has a name so you can know what that field means

// create structs with struct keyword

struct User {
  user_name: String, // In structs we tend to prefer String over &str
  active: bool,
  email: String,
}

// Of course there's a better version to do this
fn create_user(name: String, active: bool, email: String) -> User {
  // To use a struct after we define it, we create a instance of this struct
  // Then we add values to each field => key: value
  User {
    user_name: name,
    active, //  field init shorthand
    email
  }
}
// This too
fn display_username(user: &User) {
  println!("{}", user.user_name);
}

// let's do a  mutable one (Rust doesn't let us to only mutate a few field so we need to make it whole mutable)
struct Database {
  user_log: User,
  created_at_date: String
}

// create a database instance
fn create_database(user: &User, date: String) -> Database {
  let mut db = Database {
    user_log: user,
    created_at_date: date
  };
  db.created_at_date = "03-19-2020".to_string(); // MM-DD-YYYY
  db
}
fn display_db_registry(db: Database){
  println!("User: \n {} \nCreated at: \n {}", db.user_log.user_name, db.created_at_date)
}
pub fn start() {
  let user1: User = create_user(String::from("spaghetti"), true, String::from("spaghettimail@hotmail.com"));
  let db1 = create_database(&user1, String::from("05-02-2021"));
  display_db_registry(db1);
}