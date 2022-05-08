fn solve(arr: &[i32]) -> Vec<i32> {
    let half = arr.len() / 2;
    let mut sorted = arr.to_vec();
    sorted.sort_unstable();
    let mins = &sorted[..half];
    let mut maxs = sorted[half..].to_vec();
    maxs.reverse();
    let mut result = Vec::new();
    for i in 0..half {
        result.push(maxs[i]);
        result.push(mins[i]);
    }
    if arr.len() % 2 == 1 {
        result.push(maxs[half]);
    }
    result
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn basic_tests() {
        assert_eq!(solve(&vec![15, 11, 10, 7, 12]), vec![15, 7, 12, 10, 11]);
        assert_eq!(solve(&vec![91, 75, 86, 14, 82]), vec![91, 14, 86, 75, 82]);
        assert_eq!(solve(&vec![84, 79, 76, 61, 78]), vec![84, 61, 79, 76, 78]);
        assert_eq!(
            solve(&vec![52, 77, 72, 44, 74, 76, 40]),
            vec![77, 40, 76, 44, 74, 52, 72]
        );
        assert_eq!(
            solve(&vec![1, 6, 9, 4, 3, 7, 8, 2]),
            vec![9, 1, 8, 2, 7, 3, 6, 4]
        );
        assert_eq!(
            solve(&vec![78, 79, 52, 87, 16, 74, 31, 63, 80]),
            vec![87, 16, 80, 31, 79, 52, 78, 63, 74]
        );
    }
}
