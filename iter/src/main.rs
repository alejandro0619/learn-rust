fn main() {
  using_other_other_iterator_traits_methods();
   
}
#[derive(PartialEq, Debug)]
struct Color{
    color: String
}
#[derive(PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: Color
}
fn in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter( | s | {
        s.size == shoe_size
    }).collect()
}
impl Shoe{
    fn filter_by_size(){
        let shoes = vec![
            Shoe {
                size: 10,
                style: Color {
                    color: String::from("Blue")
                }
            },
            Shoe {
                size: 25,
                style: Color {
                    color: String::from("Yellow")
                }
            },
            Shoe {
                size: 10,
                style: Color {
                    color: String::from("Orange")
                }
            },
        ];
        let in_my_size = in_my_size(shoes, 10);
        println!("{:#?}", in_my_size);
    }
}

//creating my own iter:
struct Counter {
    count: u32
}
impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else{
             None
        }
    }
}
    fn using_other_other_iterator_traits_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("{}", sum);
    }
