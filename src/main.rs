fn sum_of_multiples_of_3_or_5_under_threshold(threshold: i32) -> i32 {
    let blocks = threshold/30;
    let rem = threshold % 30;

    let mut x = 0;
    for block_id in 0..blocks {
        let block_offset = block_id * 30;
        x += 225 + block_offset * 14;
    }

    for i in 1..rem {
        let v = blocks * 30 + i;
        if v % 3 == 0 || v % 5 == 0 {
            x += v;
        }
    }

    x
}

fn main() {
    println!("{}", sum_of_multiples_of_3_or_5_under_threshold(1000));
}