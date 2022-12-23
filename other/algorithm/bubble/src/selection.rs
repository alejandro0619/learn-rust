use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
  fn sort<T>(slice: &mut [T]) 
    where 
      T: Ord, {
        // We are going to walk through the array
        // Find the smallest element of the list and
        // Stick it into the front.
        // And so on until the array is already sorted
          for unsorted in 0..slice.len() {
              let mut smallest = unsorted;
              for i in (unsorted)..slice.len(){
                if slice[i] < slice[smallest] {
                  smallest = i;

                }
              }
              if unsorted != smallest {
                slice.swap(unsorted, smallest);
              }
          }
      }
}
#[test]
fn selection_test() {
    let mut my_vec = vec![1, 9, 6];
    super::sort::<_, SelectionSort>(&mut my_vec);
    assert_eq!(
        my_vec,
        &[1, 6, 9]
    )
}