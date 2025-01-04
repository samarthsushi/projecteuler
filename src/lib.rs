mod utils;
use utils::is_prime;

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

pub fn q7() -> u64 {
    let mut primes = vec![2,3];
    let mut curr = 5;
    let mut i = 3;
    loop {
        if i > 10001 { break; }
        if is_prime(curr, &primes) { primes.push(curr); i+=1; }
        curr+=2;
    }
    *primes.iter().last().unwrap()
}

pub fn q33() -> i32 {
    let primes = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47];
    let mut prime_factors = vec![vec![0;15]];

    fn reduce_fraction(num: i32, denom: i32, prime_factors: &Vec<Vec<i32>>) -> Vec<i32> {
        let a = &prime_factors[num as usize];
        let b = &prime_factors[denom as usize];

        a   .iter()
            .zip(b.iter())
            .map(|(a,b)| (a - b).abs())
            .collect()
    }
    for x in 1..100 {
        // prime factorize all nums from 0 to 99 in terms of the numbers in `primes` slice 
        // (a 15 len array that stores how many times that number appears in prime factor of x)
        // i.e. 12 is [2,1,0,0,0,0,0,0,0,0,0,0,0,0,0]
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
    for (i,p) in prime_factors.iter().enumerate() {
        println!("{i}. {:?}", p);
    }
    let mut curious = vec![];

    // remove actual division from this step, access the `prime factors` vec for both numerator and denom and cancel the common terms, to get smallest form of fraction
    for xy in 10..99 {
        let y = xy%10;
        let x = xy/10;
        // check against tens digit
        for z in 0..10 {
            let zx = format!("{z}{x}").parse::<i32>().unwrap();
            if reduce_fraction(xy, zx, &prime_factors) == reduce_fraction(y, z, &prime_factors) {
                curious.push((xy, zx));
            }
        }

        // check agaisnt ones digit
        for z in 0..10 {
            let yz = format!("{y}{z}").parse::<i32>().unwrap();
            if reduce_fraction(xy, yz, &prime_factors) == reduce_fraction(x, z, &prime_factors) {
                curious.push((xy, yz));
            }
        }
    }

    println!("{:?}", curious);
    curious[0].0
}

pub fn q13() -> u64 {
    let q = std::fs::read_to_string("data/q13.txt").unwrap();
    todo!();
}