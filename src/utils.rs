pub fn is_prime_u64(x: u64) -> bool {
    if x == 1 { return false; }
    else if x < 4 { return true; }
    else if x % 2 == 0 { return false;}
    else if x < 9 { return true; }
    else if x % 3 == 0 { return false; }
    else {
        let r = (x as f64).sqrt() as u64;
        let mut f = 5;
        while f <= r {
            if x % f == 0 { return false; }
            if x % (f+2) == 0 { return false; }
            f+=6;
        }
        true
    }
}