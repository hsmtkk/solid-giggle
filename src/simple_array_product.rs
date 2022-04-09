fn solve(vecs: &[Vec<i32>]) -> i32 {
    *solve2(vecs).iter().max().unwrap()
}

fn solve2(arrays: &[Vec<i32>]) -> Vec<i32> {
    if arrays.len() == 1 {
        return arrays[0].to_vec();
    }
    let next = solve2(&arrays[1..]);
    let mut result = Vec::new();
    for x in &arrays[0] {
        for y in &next {
            result.push(x * y);
        }
    }
    result
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(solve(&[vec![1, 2], vec![3, 4]]), 8);
        assert_eq!(solve(&[vec![10, -15], vec![-1, -3]]), 45);
        assert_eq!(solve(&[vec![-1, 2, -3, 4], vec![1, -2, 3, -4]]), 12);
        assert_eq!(
            solve(&[vec![-11, -6], vec![-20, -20], vec![18, -4], vec![-20, 1]]),
            17600
        );
        assert_eq!(solve(&[vec![14, 2], vec![0, -16], vec![-12, -16]]), 3584);
        assert_eq!(solve(&[vec![-3, -4], vec![1, 2, -3]]), 12);
    }
}
