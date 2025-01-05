use projecteuler::{q33,q7};
#[allow(unused_imports)]
use projecteuler::{q1,q2,q3,q6,q13,q15};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let x = q33();
    let dur = start.elapsed();
    println!("{:?}\n{:?}", x, dur);
}