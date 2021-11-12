use std::time::Instant;

pub fn run_loop() {
  let start_value :i128 = 0;
  let end_value :i128 = 999999;
  let start = Instant::now();
  let mut x = start_value;
  while x <= end_value {
    x += 1;
  }
  let end = Instant::now();
  println!("Iteration from {:?} to {:?} took: {:?}", start_value, end_value, end.duration_since(start));
}