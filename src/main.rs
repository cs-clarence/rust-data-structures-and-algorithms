mod libs;
use std::time::{Instant};
use libs::algorithms::{sort::*};
use rand::prelude::*;

fn benchmark(bench_msg: &str, func: &dyn Fn() -> ()) {
  let start_time = Instant::now();
  println!("started benchmark: {}", bench_msg);
  func();
  let end_time = start_time.elapsed();
  println!(
    r#"
--------> benchmark ended: {} took [{:?}]
-----------------------------------------"#,
    bench_msg, end_time
  );
}

fn main() {
  let mut arr = vec![0];
  let mut gen = thread_rng();
  for &until in &[100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 5000] {
    for i in 1..until {
      arr.push(gen.gen());
    }

    benchmark(&format!("qsort with {} elements", until), &|| {
      qsort(&arr);
    });
    benchmark(&format!("sshort with {} elements", until), &|| {
      shshort(&arr);
    });
    benchmark(&format!("isort with {} elements", until), &|| {
      isort(&arr);
    });
    benchmark(&format!("ssort with {} elements", until), &|| {
      ssort(&arr);
    });
  }

  let mut sample_vec = vec![0];
  for i in 0..10 {
    sample_vec.push(gen.gen());
  }

  println!("{:?}", qsort(&sample_vec));
  println!("{:?}", ssort(&sample_vec));
  println!("{:?}", isort(&sample_vec));
  println!("{:?}", shshort(&sample_vec));
}
