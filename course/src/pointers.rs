// reference pinter ' point to a resource in memoryÑ

pub fn run() {
  // primitive array:
  let arr1 = [1,2,3];
  let arr2 = arr1;

  println!("Values: {:?}", (arr1, arr2));

  // With non-primitives, if you assifn another vfaraible to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource.
  let vec1 = vec![1,2,3];
  let vec2 = &vec1;
  println!("values of the vector: {:?}", (&vec1, vec2));
}