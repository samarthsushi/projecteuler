#[allow(unused_imports)]
use projecteuler::{q1,q2,q3,q6,q13,q15,q33,q4,q7};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let x = q4();
    let dur = start.elapsed();
    println!("{}\n{:?}", x, dur);
}