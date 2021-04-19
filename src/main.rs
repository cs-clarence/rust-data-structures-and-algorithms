mod libs;
use libs::algorithms::{sort::*};

fn main() {
  let arr = vec![0, 1, 2, 3, 4, 5, 6];
  println!("{:?}", ssort(&arr));
  println!("{:?}", qsort(&arr));
}
