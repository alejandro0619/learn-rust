use super::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
  fn sort<T>(slice: &mut [T])
    where 
      T: Ord, {
        /*
        We set a variable swapped to manage
        Whether the loop should continue or stop
        We set it to true, then in each iteration
        We set the value to false, and we'll
        keep iterating as long as didn't make the
        whole walk along the slice.
        */
        let mut swapped = true;
        while swapped {
          swapped = false;
          for i in 1..slice.len() {
            if slice[i - 1] > slice[i] {
              slice.swap(i - 1, i);
              swapped = true;
            }
          }
        }
      }
}
#[test]
fn bubble_test() {
    let mut my_vec = vec![1, 9, 6];
    super::sort::<_, Bubblesort>(&mut my_vec);
    assert_eq!(
        my_vec,
        &[1, 6, 9]
    )
}
