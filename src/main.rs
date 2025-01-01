use projecteuler::{q1,q2,q3};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let x = q3();
    let dur = start.elapsed();
    println!("{}\n{:?}", x, dur);
}