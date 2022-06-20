fn generate_pairs(m: i16, n: i16) -> Vec<(i16, i16)> {
    let mut result = Vec::new();
    for a in m..n+1 {
        for b in a..n+1 {
            result.push((a, b));
        }
    }
    result
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test_case() {
        assert_eq!(generate_pairs(2, 4), vec![(2, 2), (2, 3), (2, 4), (3, 3), (3, 4), (4, 4)]);
        assert_eq!(generate_pairs(0, 1), vec![(0, 0), (0, 1), (1, 1) ]);
        assert_eq!(generate_pairs(0, 0), vec![(0, 0)]);
    }
}

