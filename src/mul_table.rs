fn multiplication_table(size: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for row in 0..size {
        let mut line = Vec::new();
        for column in 0..size {
            line.push((row + 1) * (column + 1));
        }
        result.push(line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::multiplication_table;

    #[test]
    fn test1() {
        let want = vec![vec![1]];
        let got = multiplication_table(1);
        assert_eq!(want, got);
    }

    #[test]
    fn test2() {
        let want = vec![vec![1, 2], vec![2, 4]];
        let got = multiplication_table(2);
        assert_eq!(want, got);
    }

    #[test]
    fn test3() {
        let want = vec![vec![1, 2, 3], vec![2, 4, 6], vec![3, 6, 9]];
        let got = multiplication_table(3);
        assert_eq!(want, got);
    }
}
