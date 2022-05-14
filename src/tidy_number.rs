fn tidy_number(n: u64) -> bool {
    let ds = num_to_digits(n);
    for i in 1..ds.len() {
        if ds[i] < ds[i - 1] {
            return false;
        }
    }
    true
}

fn num_to_digits(num: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut n = num;
    loop {
        result.push(n % 10);
        n /= 10;
        if n == 0 {
            break;
        }
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tidy_number(12), true);
        assert_eq!(tidy_number(102), false);
        assert_eq!(tidy_number(9672), false);
        assert_eq!(tidy_number(2789), true);
        assert_eq!(tidy_number(2335), true);
    }

    #[test]
    fn test_num_to_digits() {
        assert_eq!(vec![1, 0, 2], num_to_digits(102));
        assert_eq!(vec![9, 6, 7, 2], num_to_digits(9672));
    }
}
