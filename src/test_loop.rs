use std::time::Instant;

pub fn run_loop() {
  let start_value :i128 = 0;
  let end_value :i128 = 999999;
  let start = Instant::now();
  for x in start_value..end_value {
    println!("{}", x);
  }
  let duration = start.elapsed();
  let elapsed = duration.as_secs() as f64;
  println!("Iteration from {:?} to {:?} took: {:?}", start_value, end_value, elapsed);
}