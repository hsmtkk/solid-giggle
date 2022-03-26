fn prev_mul_three(n: u64) -> Option<u64> {
    let orig = n.to_string();
    for i in 0..orig.len() {
        let ds = &orig[..orig.len() - i];
        let m: u64 = ds.parse().unwrap();
        if m % 3 == 0 {
            return Some(m);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::prev_mul_three;

    #[test]
    fn test_prev_mul_three() {
        let input_want = vec![
            (1, None),
            (25, None),
            (36, Some(36)),
            (1244, Some(12)),
            (952406, Some(9)),
        ];
        for (input, want) in input_want {
            let got = prev_mul_three(input);
            assert_eq!(want, got);
        }
    }
}
