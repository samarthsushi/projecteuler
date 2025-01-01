fn sum_of_multiples_of_3_or_5_under_threshold(threshold: i32) -> i32 {
    let n3 = (threshold-1)/3;
    let n5 = (threshold-1)/5;
    let n15 = (threshold-1)/15;
    (n3*(2*3 + (n3-1)*3))/2 + (n5*(2*5 + (n5-1)*5))/2 - (n15*(2*15 + (n15-1)*15))/2
}

fn main() {
    println!("{}", sum_of_multiples_of_3_or_5_under_threshold(1000));
}