use std::usize;

/// Implementation For Binary Search
pub fn bsearch<T: PartialOrd + PartialEq>(a: &[T], key: &T) -> Option<usize> {
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
