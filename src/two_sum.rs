fn two_sum(ns: &[i32], target: i32) -> (usize, usize) {
    for i in 0..ns.len() - 1 {
        for j in i + 1..ns.len() {
            if ns[i] + ns[j] == target {
                return (i, j);
            }
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test0() {
        let want = (0, 2);
        let got = two_sum(&vec![1, 2, 3], 4);
        assert_eq!(want, got);
    }

    #[test]
    fn test1() {
        let want = (1, 2);
        let got = two_sum(&vec![1234, 5678, 9012], 14690);
        assert_eq!(want, got);
    }

    #[test]
    fn test2() {
        let want = (0, 1);
        let got = two_sum(&vec![2, 2, 3], 4);
        assert_eq!(want, got);
    }
}
