use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
  fn sort<T>(slice: &mut [T]) 
    where 
      T: Ord, {
        /*
        We divide the slice into sorted and not-sorted.
        Where sorted is initialized empty
        And not sorted is initialized with the entire list 
        */
        //  [sorted | not sorted]
        // We walk through the slice
        // [1..slice.len] because a slice of a single element
        // Is already sorted
          for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in a sorted 
            // location in slice[..=unsorted]
            // [1 3 4 | 2]
            // We'll move the element to the left until
            // The next number is smaller than the unsorted.
            let mut i = unsorted;
            while i > 0 && slice[i - 1] > slice[i] {
              slice.swap(i - 1, i);
              i -= 1;
            }
          }
      }
}