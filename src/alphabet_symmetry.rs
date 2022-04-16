const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";

fn count_symmetries(words: &[&str]) -> Vec<usize> {
    words.iter().map(count_symmetry).collect()
}

fn count_symmetry(word: &&str) -> usize {
    let low = word.to_lowercase();
    let mut count = 0;
    for (x, y) in std::iter::zip(ALPHABETS.chars(), low.chars()) {
        if x == y {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::count_symmetries;

    #[test]
    fn test_count_symmetries() {
        let test_cases = vec![
            (vec!["abode", "ABc", "xyzD"], vec![4, 3, 1]),
            (vec!["abide", "ABc", "xyz"], vec![4, 3, 0]),
            (
                vec!["IAMDEFANDJKL", "thedefgh", "xyzDEFghijabc"],
                vec![6, 5, 7],
            ),
            (vec!["encode", "abc", "xyzD", "ABmD"], vec![1, 3, 1, 3]),
        ];
        for test_case in test_cases {
            assert_eq!(test_case.1, count_symmetries(&test_case.0));
        }
    }
}
