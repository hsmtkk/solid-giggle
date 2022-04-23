fn sum_even_fibonacci(limit: u32) -> u32 {
    let fib_seq = fibonacci_sequence(limit);
    fib_seq.iter().filter(|&x| x % 2 == 0).sum()
}

fn fibonacci_sequence(limit: u32) -> Vec<u32> {
    let mut x = 1;
    let mut y = 2;
    let mut seq = vec![1, 2];
    loop {
        let z = x + y;
        if z > limit {
            return seq;
        }
        seq.push(z);
        x = y;
        y = z;
    }
}

#[cfg(test)]
mod tests {
    use super::sum_even_fibonacci;

    #[test]
    fn test0() {
        assert_eq!(10, sum_even_fibonacci(8));
    }

    #[test]
    fn test1() {
        assert_eq!(60696, sum_even_fibonacci(111111));
    }

    #[test]
    fn test2() {
        assert_eq!(4613732, sum_even_fibonacci(8675309));
    }
}
