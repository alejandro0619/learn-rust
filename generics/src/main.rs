fn main() {
    // PointTwo:
    let p1 = PointTwo { x: 5, y:10.4};
    let p2 = PointTwo { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // trais:
    summary();
}


//generics in function:


// generics in structs:
struct _Point<T>{
    x: T,
    y: T
}

// to define a struct where x and y are generics but in differents types, let's change its deifnition:

struct PointTwo<T, U> {
    x: T,
    y: U
}

impl<T, U>PointTwo<T, U>{
    fn mixup<V, W>(self, other: PointTwo<V, W>) -> PointTwo<T, W> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
}

//we can implement methods for generics structs and enums:
impl<T> _Point<T> {
    fn _x (&self) -> &T{
        &self.x
    }
}
//impl fn to all typs of Point<f32>
impl _Point<f32>{
    fn _distanc_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// let's talk about taits:
/*
Supongamos que tenemos múltiples estructuras que contienen varios tipos y cantidades de texto:
una estructura NewsArticle que contiene una historia de noticias archivada en una ubicación
particular y un Tweet que puede tener un máximo de 280 caracteres junto con metadatos que
indican si se trataba de un nuevo tweet, un retweet o una respuesta a otro tweet.
Queremos crear una biblioteca de agregadores de medios que pueda mostrar resúmenes de datos
que puedan estar almacenados en una instancia de NewsArticle o Tweet. Para hacer esto,
necesitamos un resumen de cada tipo, y necesitamos solicitar ese resumen llamando a un método de
resumen en una instancia. Definamos un trait Summary que realice este procedimiento:
*/

fn summary(){
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String{
            format!("{}, by {}, ({})", self.headline, self.author, self.location)

        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool
    }

    impl Summary for Tweet{
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // call:
    let tweet: Tweet =  Tweet {
        username: String::from("spaghetti.rs"),
        content: String::from("I fucking hate you Rustlang, gonna AFK"),
        reply: false,
        retweet:false
    };
    println!("1 new tweet: {}", tweet.summarize());
}

