fn reduce(nums: &[u32; 2]) -> [u32; 2] {
    let g = gcd(nums[0], nums[1]);
    [nums[0] / g, nums[1] / g]
}

fn gcd(m: u32, n: u32) -> u32 {
    if m < n {
        gcd(n, m)
    } else if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

#[cfg(test)]
mod tests {
    use super::reduce;

    #[test]
    fn test_reduce() {
        assert_eq!([3, 1], reduce(&[60, 20]));
        assert_eq!([2, 3], reduce(&[80, 120]));
        assert_eq!([2, 1], reduce(&[4, 2]));
        assert_eq!([3, 8], reduce(&[45, 120]));
        assert_eq!([1000, 1], reduce(&[1000, 1]));
    }
}
