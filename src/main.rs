mod libs;
use std::time::{Instant};
use libs::algorithms::{search::*, sort::*};
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
    for _ in 1..until {
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
    benchmark(&format!("bsort with {} elements", until), &|| {
      bsort(&arr);
    });
  }

  let mut sample_vec = vec![0];
  for _ in 0..10 {
    sample_vec.push(gen.gen());
  }

  sample_vec = qsort(&sample_vec);

  println!("{:?}", sample_vec);

  if let Some(v) = fnbsearch(&sample_vec, &sample_vec[sample_vec.len() / 2]) {
    println!("fnbsearch: {}", v);
  }
  if let Some(v) = lsearch(&sample_vec, &sample_vec[sample_vec.len() / 2]) {
    println!("lsearch: {}", v);
  }
  if let Some(v) = bsearch(&sample_vec, &sample_vec[sample_vec.len() / 2]) {
    println!("bsearch: {}", v);
  }
}
