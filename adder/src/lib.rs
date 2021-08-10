#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another(){
        panic!("aaaaaah")
    }
    #[derive(Debug)]
    struct Rect {
        w: u32,
        h: u32
    }
    impl Rect {
        fn can_hold(&self, other: &Rect) -> bool {
            self.w > other.w && self.h > other.h
        }
    }
    #[test]
    fn can_hold_smaller(){
        let larger = Rect{
            w: 8,
            h: 7
        };
        let smaller = Rect {
            w: 5, 
            h: 1
        };
        assert!(larger.can_hold(&smaller));
    }
    pub struct Guess {
        value: u32
    }
    impl Guess {
        fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guees value must be between 1 and 100, got {}", value);
            }
            Guess {
                value
            }
        }
    }
}
