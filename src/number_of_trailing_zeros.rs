use num_bigint::{ToBigUint, BigUint};

fn zeros(n: u64) -> u64 {
    let mut f = fib(n);
    let mut count = 0;
    loop {
        let m = &f % 10u32;
        if m != 0.to_biguint().unwrap() {
            break;
        }
        count += 1;
        f /= 10u32;
    }
    count
}

fn fib(n: u64) -> BigUint {
    let mut ns = Vec::new();
    for i in 1..n+1 {
        ns.push(i.to_biguint().unwrap());
    }
    ns.iter().fold(1.to_biguint().unwrap(), mul)
}

fn mul(acc:BigUint, n:&BigUint) -> BigUint {
    acc * n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}