pub fn q1() -> i32 {
    let threshold = 1000;
    let n3 = (threshold-1)/3;
    let n5 = (threshold-1)/5;
    let n15 = (threshold-1)/15;
    (n3*(2*3 + (n3-1)*3))/2 + (n5*(2*5 + (n5-1)*5))/2 - (n15*(2*15 + (n15-1)*15))/2
}

pub fn q2() -> i32 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 2;

    while a < 4_000_000 {
        if a % 2 == 0 {
            sum += a;
        }
        let t = a + b;
        a = b;
        b = t;
    }
    sum
}

pub fn q3() -> i64 {
    let mut num = 600851475143_i64;
    let num_sqrt = (num as f64).sqrt() as i64;
    let mut i = 2;
    
    while num > 1 {
        if num % i == 0 {
            num /= i;
        } else if i > num_sqrt {
            i = num;
        } else {
            i += 1;
        }
    }
    i
}

pub fn q6() -> i32 {
    // (n(n+1)/2)^2 - n(n+1)(2n+1)/6 simplified
    let n: i32 = 100;
    (3*(n.pow(4)) + 2*(n.pow(3)) - 3*(n.pow(2)) - 200)/12
}

pub fn q15() -> u64 {
    // 40!/(20!*20!) because there will be 40 moves and 20 down/20 right have to be part of it
    let mut result = 1;

    for i in 0..20 {
        result = result * (40 - i) / (i + 1);
    }

    result
}

pub fn q13() -> u64 {
    let q = std::fs::read_to_string("data/q13.txt").unwrap();
    todo!();
}