#[allow(unused_imports)]
use projecteuler::{q1,q2,q3,q6,q13,q15,q33,q5,q7,q4,q8,q10,q26,q35,q53};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let x = q53();
    let dur = start.elapsed();
    println!("{}\n{:?}", x, dur);
}