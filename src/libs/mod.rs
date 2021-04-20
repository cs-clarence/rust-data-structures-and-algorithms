pub mod algorithms {
  pub mod sort {
    fn find_min<T>(v: &[T]) -> usize
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
    /// Selection Sort Implementation
    pub fn ssort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      // Pretty basic steps
      // STEP 0. Create a new empty output array
      // STEP 2. Find the index of the value with the least value in the input array
      // STEP 3. Remove the the element with the least value and put it in the output array
      // STEP 4. Repeat steps until the input array runs out of element
      // STEP 5. Return the output array
      let mut v = v.to_vec(); //
      let mut ret_v = vec![]; // becomes the sorted vector

      while v.len() > 0 {
        ret_v.push(v.remove(find_min(&v)));
      }

      ret_v
    }

    /// Insertion Sort Implementation
    pub fn isort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      /*
      The idea is to sort the items in place
      With a nested loop (2 loops)
      for i - will advance the the index forward start at index 1
        for j - will start at index i, (j = i), then start decrease j every iteration
        condition 1 = will check if j is greater than 0 (not at the start of the array)
        condition 2 = then check if the element index j is has lower value than the element before it
        if it is the case, swap the value of j and the element before it

      ---
      let the input array be [0, 3, 2, 4, 1, 6, 5]
      ---
      STEP 0. Start the iteration of the outer at index 1
      [3, 0, 2, 4, 1, 6, 5]
          ^ - start here at index 1, i = 1
      ---
      STEP 1. Check if the array[i] (element at index 1) is less than array[n-1] (element preceding the element at index 1)
      And if the current index is greater than 0 (check if we're not at the first element)
      [3, 0, 2, 4, 1, 6, 5]
       ^  ^ - array[i] (array[1])
       |----- array[i-1] (array[0])
       array[i] is less than array[i-1] ?
      ---
      STEP 2. If array[i] is less than array[i-1], swap their value
      [0, 3, 2, 4, 1, 6, 5] -> after swap because 0 is less than 3
       ^  ^ - array[i] (array[i-1] previously)
       |----- array[i-1] (array[i] previously)
      ---
      STEP 3. Repeat STEP 1 to 2 until we exhaust the array
      ---
      STEP 4. When reach the last element, the whole array is sorted. Return the sorted array.
      ---
      Example array as the algorithm run overtime
      [0, 3, 2, 4, 1, 6, 5]
          ^  ^ array[i]
          |--- array[i-1]
          array[i] < array[i-1]? Yes. Swap the value
      [0, 2, 3, 4, 1, 6, 5]
       ^  ^ array[j]
       |--- array[j-1]
          array[j] < array[j-1]? No. Stop the inner looop
        advance the outer loop - repeat the steps
      */

      // Convert the array into a vector
      let mut v = v.to_vec();

      // Save the length of the array
      let n = v.len();

      for i in 1..n {
        let mut j = i;

        while j > 0 && v[j] < v[j - 1] {
          v.swap(j, j - 1);
          // let temp = v[j].clone();
          // v[j] = v[j - 1].clone();
          // v[j - 1] = temp;
          j -= 1;
        }
      }

      v
    }

    pub fn msort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      let mut v = v.to_vec();
      let mut ret_v = vec![];

      ret_v
    }

    // Implementation for Bubble Sort
    pub fn bsort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      /*

      */
      let mut v = v.to_vec();
      let mut pass = v.len() - 1;
      while pass > 0 {
        let mut i = 0;
        while i < pass {
          if v[i] > v[i + 1] {
            v.swap(i, i + 1);
          }
          i += 1;
        }
        pass -= 1;
      }

      v
    }

    /// Implementation For Shell Sort
    pub fn shshort<T: PartialOrd + Clone>(v: &[T]) -> Vec<T> {
      /*
      Shell Sort is an extension for insertion sort
      Insertion Sort can take up to N-1 exchanges to determine which index an element belongs to
      Shell Sort first partially sorts arrays with a long distance (h-sorting) first then
      the h-sort gets lower and lower every iteration meaning distance gets lower until h becomes 1
      then at that point it becomes an insertion sort but the array is is partially sorted where insertion sort shines
      which speeds up the process
       */
      let mut v = v.to_vec();
      let n = v.len();
      let mut h = 1;

      // get the largest h-sort from the length of the array
      while h < n {
        h = 3 * h + 1
      }

      while h >= 1 {
        for i in h..n {
          let mut j = i;
          while j >= h && v[j] < v[j - h] {
            v.swap(j, j - h);
            // let temp = v[j].clone();
            // v[j] = v[j - 1].clone();
            // v[j - 1] = temp;

            j -= h;
          }
        }
        h /= 3;
      }

      v
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

  pub mod search {
    use std::usize;

    /// Implementation For Binary Search
    pub fn bsearch<T: PartialOrd + PartialEq>(
      a: &[T],
      key: &T,
    ) -> Option<usize> {
      // A binary search takes in an array that is sorted in increasing order

      // find k = 6

      // l     m     h -> m = [l + (h - l) / 2] = [0 + (6 - 0) / 2] = [3]
      // 1 2 3 4 5 6 7 ---- m = k ? No. m > k ? No. l = m + 1, h = h so l = 4, h = 6
      // if m > k is true. l = l, h = m -1

      //         l m h -> m = [l + (h - l) / 2] = [4 + (6 - 4) / 2] = [5]
      // 1 2 3 4 5 6 7 ---- m = k ? Yes. Return m
      // if m = k is still false with 3 elements left, then the element does not exist in this array
      // let's assume k = 8
      // m > k? No. l = m + 1, h = h so l = 5, h = 6

      //           m
      //           l h -> m = [l + (h - l) / 2] = [5 + (6 - 5) / 2] = [5]
      // 1 2 3 4 5 6 7 ---- m = k ? No. m > k ? No. l = m + 1, h = h so l = 6, h = 6

      //             m
      //             l
      //             h -> m = [l + (h - l) / 2] = [6 + (6 - 6) / 2] = [6]
      // 1 2 3 4 5 6 7 ---- m = k ? No. m > k ? No. l = m + 1, h = h so l = 7, h = 6
      // At this point l is going to be out of bounds so the condition for the loop should be
      // l <= h, once this becomes false, the element is sure to not be in the array

      // Looped implementation of binary search
      let mut hi = a.len() - 1;
      let mut lo = 0;

      while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if a[mid] == *key {
          return Some(mid);
        } else if a[mid] < *key {
          lo = mid + 1;
        } else {
          hi = mid - 1;
        }
      }

      None
    }

    /// Implementation For Linear Search
    pub fn lsearch<T: PartialEq>(a: &[T], key: &T) -> Option<usize> {
      // This is simple enough, it doesn't need an explanation
      let mut i = 0;
      let length = a.len();
      while i < length {
        if a[i] == *key {
          return Some(i);
        };
        i += 1;
      }

      None
    }

    /// Implementation For Front and Back Search
    pub fn fnbsearch<T: PartialEq>(a: &[T], key: &T) -> Option<usize> {
      // search with even number of elements
      // f         b
      // 1 2 3 4 5 6
      //   f     b
      // 1 2 3 4 5 6
      //     f b
      // 1 2 3 4 5 6
      //     b f = stop
      // 1 2 3 4 5 6

      // search with odd number of elements
      // f           b
      // 1 2 3 4 5 6 7
      //   f       b
      // 1 2 3 4 5 6 7
      //     f   b
      // 1 2 3 4 5 6 7
      //       f
      //       b
      // 1 2 3 4 5 6 7
      //     f      = stop
      //         b  = stop
      // 1 2 3 4 5 6 7

      let mut front = 0;
      let mut back = a.len() - 1;
      while front <= back {
        if a[front] == *key {
          return Some(front);
        };
        if a[back] == *key {
          return Some(back);
        }

        front += 1;
        back -= 1;
      }

      None
    }
  }
}
