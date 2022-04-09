fn fit_in(a: u32, b: u32, m: u32, n: u32) -> bool {
    if a > m || a > n || b > m || b > n {
        return false;
    }
    let box_long = a + b;
    let box_short = a.min(b);
    let case_long = m.max(n);
    let case_short = m.min(n);
    box_long <= case_long && box_short <= case_short
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(true, 1, 2, 3, 2);
        test(false, 1, 2, 2, 1);
        test(false, 3, 2, 3, 2);
        test(false, 1, 2, 1, 2);
        test(false, 6, 5, 8, 7);
        test(false, 4,1,5,3);
    }

    fn test(expected: bool, a: u32, b: u32, m: u32, n: u32) {
        assert_eq!(
            fit_in(a, b, m, n),
            expected,
            "a={} and b={} should {}fit into m={} and n={}",
            a,
            b,
            if expected { "" } else { "NOT " },
            m,
            n
        );
    }
}
