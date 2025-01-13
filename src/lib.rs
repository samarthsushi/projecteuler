mod utils;
use utils::is_prime_u64;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::{Add, Mul, Sub};

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

pub fn q4() -> i32 {
    let mut max_palindrome = 0;
    for a in 100..1000 {
        for b in 100..1000 {
            let ab = a * b;
            let ab_vec = ab.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
            let mut ptr1 = 0;
            let mut ptr2 = ab_vec.len()-1;
            let mut is_palindrome = true;
            while ptr1 < ptr2 {
                if ab_vec[ptr1] != ab_vec[ptr2] { is_palindrome = false; }
                ptr1+=1;
                ptr2-=1;
            }
            if is_palindrome && ab > max_palindrome { max_palindrome = ab; }
        }
    }
    max_palindrome
}

pub fn q5() -> i64 {
    let factors = [19,18,17,16,15,14,13,12,11];
    let mut x = 2520;
    'outer: loop {
        for f in factors {
            if x % f != 0 { 
                x+=20; 
                continue 'outer; 
            }
        }
        break;
    }
    x
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

pub fn q7() -> u64 {
    let mut curr = 3;
    let mut i = 2;
    while i < 10001 {
        curr+=2;
        if is_prime_u64(curr) { i+=1; } 
    }
    curr
}

pub fn q8() -> u64 {
    let data = std::fs::read_to_string("data/q8.txt").unwrap();
    let vec_chars: Vec<char> = data
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    println!("{:?}\n{}", vec_chars, vec_chars.len());
    let mut ptr = 0;
    let mut max_product = 0;
    while ptr < vec_chars.len()-13 {
        let x = vec_chars[ptr..ptr+13].iter().map(|a| a.to_digit(10).unwrap() as u64).product();
        if x > max_product { max_product = x; }
        ptr+=1;
    }
    max_product
}

pub fn q10() -> u64 {
    let mut sum = 5;
    for x in (5..2_000_000).step_by(2) {
        if is_prime_u64(x) { sum += x; }
    }
    sum
}

pub fn q26() -> u64 {
    fn cycle_length(n: u64) -> u64 {
        let mut remainders = vec![-1; n.try_into().unwrap()];
        let mut value = 1;
        let mut position = 0;

        while value != 0 {
            if remainders[value] != -1 {
                return position - remainders[value] as u64;
            }
            remainders[value] = position as isize;
            value = (value * 10) % n as usize;
            position += 1;
        }
        0
    }

    let mut max_cycle = 7;
    for x in 11..1000 {
        if is_prime_u64(x) {
            if max_cycle < cycle_length(x) { max_cycle = x; };
        }
    }
    max_cycle
}

pub fn q35() -> u64 {
    fn valid_rotations(mut x: u64) -> bool {
        let mut digits = Vec::new();
        while x > 0 {
            digits.push(x % 10);
            x /= 10;
        }
        let len = digits.len();
        digits.reverse();
        for i in 0..len {
            let r: Vec<u64> = digits
                .iter()
                .cycle()
                .skip(i)
                .take(len)
                .cloned()
                .collect();
            let rotated_number = r
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit);
            if !is_prime_u64(rotated_number) { return false; }
        }
        true
    }
    
    let mut count = 13;
    for x in (101..1_000_000).step_by(2) {
        if is_prime_u64(x) { 
            if valid_rotations(x){
                count+=1;
            }
        }
    }
    count
}

pub fn q33() -> (i32, i32) {
    // answer: 16/64 , 19/95 , 26/65 , 49/98
    let primes = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47];
    let mut prime_factors = vec![vec![0;15]];

    fn reduce_fraction(num: i32, denom: i32, prime_factors: &Vec<Vec<i32>>) -> Vec<i32> {
        let a = &prime_factors[num as usize];
        let b = &prime_factors[denom as usize];

        a   .iter()
            .zip(b.iter())
            .map(|(a,b)| (a - b))
            .collect()
    }
    for x in 1..100 {
        let mut x_clone = x;
        let mut temp = vec![0;15];
        for (i,p) in primes.iter().enumerate() {
            loop {
                if x_clone % p == 0 {
                    temp[i] += 1;
                    x_clone /= p;
                } else {
                    break;
                }
            }
        }
        prime_factors.push(temp);
    }
    let mut curious = vec![];
    for xy in 10..99 {
        let y = xy%10;
        let x = xy/10;
        for z in x..10 {
            let zx = z*10 + x;
            if zx < xy { continue; }
            let xy_by_zx = reduce_fraction(xy, zx, &prime_factors);
            if xy_by_zx.iter().all(|&a| a == 0) { continue; }
            if xy_by_zx == reduce_fraction(y, z, &prime_factors) { curious.push((y, z)); }
        }

        for z in 0..10 {
            let yz = y*10 + z;
            if yz < xy { continue; }
            let xy_by_yz = reduce_fraction(xy, yz, &prime_factors);
            if xy_by_yz.iter().all(|&a| a == 0) { continue; }
            if xy_by_yz == reduce_fraction(x, z, &prime_factors) { curious.push((x, z)); }
        }
    }
    curious.iter().fold((1,1), |acc: (i32, i32), &(x,y)| (acc.0 * x, acc.1 * y))
}

pub fn q13() -> u64 {
    let q = std::fs::read_to_string("data/q13.txt").unwrap();
    todo!();
}

pub fn q28() -> u64 {
    let mut sum = 25;
    let mut x = 9;

    for side_length in (5..=1001).step_by(2) {
        let side_length_decr = side_length - 1;
        x += side_length_decr;
        sum += 4*x + 6*(side_length_decr);
        x += 3*(side_length_decr);
    }

    sum
}

pub fn q53() -> u128 {
    fn selection(n: u128, r: u128) -> u128 {
        let r = r.min(n - r);
        let mut result = 1;

        for i in 0..r {
            result = result * (n - i) / (i + 1);
        }
    
        result
    }
    let mut count = 0;
    for n in 23..101 {
        for r in 1..n {
            if selection(n, r) > 1_000_000 { count += 1; }
        }
    }
    count
}

pub fn q66() -> u64 {
    // pell's equation is: x^2 - Dy^2 = 1
    fn solve_pell(d: u64) -> Option<(BigUint, BigUint)> {
        let sqrt_d = (d as f64).sqrt() as u64;
        // if D is a perfect square, no solution is possible
        if sqrt_d * sqrt_d == d {
            return None;
        }
    
        let mut m = 0;
        let mut d_k = 1;
        let mut a_k = sqrt_d;
    
        let mut num1 = BigUint::one();
        let mut num = BigUint::from(a_k);
        let mut denom1 = BigUint::zero();
        let mut denom = BigUint::one();
    
        loop {
            let lhs = &num * &num;
            let rhs = &denom * &denom * d;
            if lhs == rhs.add(BigUint::one()) {
                return Some((num, denom));
            }
    
            m = d_k * a_k - m;
            d_k = (d - m * m) / d_k;
            a_k = (sqrt_d + m) / d_k;
    
            let next_num = BigUint::from(a_k) * &num + &num1;
            let next_denom = BigUint::from(a_k) * &denom + &denom1;
    
            num1 = num;
            num = next_num;
    
            denom1 = denom;
            denom = next_denom;
        }
    }
    let mut largest_x = BigUint::zero();
    let mut result_d = 0;

    for d in 2..=1000 {
        if let Some((x, _y)) = solve_pell(d) {
            if x > largest_x {
                largest_x = x.clone();
                result_d = d;
            }
        }
    }

    result_d
}