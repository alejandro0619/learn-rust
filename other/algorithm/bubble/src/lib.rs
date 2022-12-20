pub mod bubble;
pub mod insertion;

/*
We implement a trait `Sorter` that has a function sort that receives a Generic over T that implements
the Ord (total Ord) trait and receives a mutable slice over T.
*/
pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

/*
A funtion that receives generics over T and S
where T implements Ord
and S is a type that implements our own trait
*/
pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

//mod bubble
#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;

    impl Sorter for  StdSorter {
        fn sort<T>(slice: &mut [T])
            where
                T: Ord {
                    slice.sort()
                }
    }
    #[test]
    fn it_works() {
        let mut my_vec = vec![1, 9, 6];
        sort::<_, StdSorter>(&mut my_vec);
        assert_eq!(
            my_vec,
            &[1, 6, 9]
        )
    }
}
