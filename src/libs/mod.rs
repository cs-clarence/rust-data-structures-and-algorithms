pub mod algorithms {
  pub mod sort {
    pub fn find_min<T>(v: &[T]) -> usize
    where
      T: PartialOrd,
    {
      let mut least: usize = 0;
      let mut min = &v[0];

      for (index, item) in v.iter().enumerate() {
        if item < min {
          min = item;
          least = index;
        }
      }

      return least;
    }
    pub fn ssort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      let mut v = v.to_vec(); //
      let mut ret_v = vec![]; // becomes the sorted vector

      while v.len() > 0 {
        ret_v.push(v.remove(find_min(&v)));
      }

      ret_v
    }

    pub fn isort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      let mut v = v.to_vec();
      let mut ret_v = vec![];
      let mut n = v.len();

      for i in 1..n {}

      ret_v
    }

    pub fn msort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      let mut v = v.to_vec();
      let mut ret_v = vec![];

      ret_v
    }

    pub fn bsort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      let mut v = v.to_vec();
      let mut ret_v = vec![];

      ret_v
    }

    /// Quick Sort Implementation
    pub fn qsort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      /*
      Steps with illustration:
      ---
      assume array [3, 6, 2, 4, 7, 1, 5] as an example
      STEP 0. Check if the array consist of only one or zero elements, return the vector early if yes
      ---
      [3, 6, 2, 4, 7, 1, 5]
                ^ pivot element is at index 5
      [3, 6, 2, 7, 1, 5] -> removed the pivot element
      STEP 1. Select a pivot element (named pivot) and remove it from the array
      the middle element is a good candidate for a pivot
      ---
      [3, 6, 2, 7, 1, 5] -> current array with the pivot removed
      pivot value is 4
      [3, 2, 1] -> lesser_eq than 4 ------- [6, 7, 5] -> greater than 4
      STEP 2. Group the elements lesser or equal (named lesser_eq) to the pivot value to the left
      and greater(named greater) to the right
      ---
      let sorted_lesser_eq = qsort(lesser_eq)
      let sorted_greater = qsort(greater)
      STEP 3. Recursively call this function on lesser_eq and greater to sort them
      ---
      sorted_lesser_eq + pivot + sorted_greater
      STEP 4. Combine the sorted lesser_eq + pivot + sorted greater into a vector (named output)
      ---
      STEP 5. Return the output
      */

      // create a vector from the array for ease of use
      let mut v = v.to_vec();
      if v.len() <= 1 {
        return v;
      }

      // get the pivot item from the middle of the vector
      let pivot = v.remove(v.len() / 2);

      // create a vector store where the
      // lesser or equal elements and greater elements will be stored
      let mut lesser_eq = vec![];
      let mut greater = vec![];

      // split the vector into lesser_eq and greater
      while v.len() != 0 {
        let got = match v.pop() {
          Some(v) => v,
          None => break,
        };

        if got <= pivot {
          lesser_eq.push(got);
        } else {
          greater.push(got);
        }
      }

      // create a vector from lesser_eq group + pivot + greater
      // output will contain the sorted vector
      let mut output = qsort(&lesser_eq);
      let mut greater = qsort(&greater);
      output.push(pivot);
      output.append(&mut greater);

      // return the sorted vector
      output
    }
  }
}
