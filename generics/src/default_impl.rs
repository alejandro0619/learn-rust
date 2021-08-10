fn main(){
    pub trait Summary{
        // trait method
        fn summarize_autor(&self) -> String;

        // default method:
        fn summarize (&self) -> String{
            format!("read more from {}", self.summarize_autor())
        }
    }
    
    pub struct _NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool
    }
    impl Summary for Tweet {
        fn summarize_autor(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from( "of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    //as params:
    fn _notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // traits bounds:
    // (Specifying Multiple Trait Bounds with the + Syntax)
   /*
    fn _notify2<T: Summary + Display >(item: &T){ // or (item: &(impl Summary + Summary)){ }
        println!("Breaking news {}", item.summarize())
    } */

    /*
    Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. So instead of writing this:
    */
   /*
    fn _some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
         stuff...
    }
     we can do something Like this
    fn _another_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {

          }
    */

    //Returning Types that Implement Traits:
 
    fn _returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
    


}