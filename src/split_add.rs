use std::iter::zip;

fn split_add(numbers: &[i32], repeat: i32) -> Vec<i32> {
    let mut result = numbers.to_vec();
    for _i in 0..repeat {
        if result.len() == 1 {
            break;
        }
        let (left, right) = split(&result);
        result = add(&left, &right);
    }
    result
}

fn split(numbers: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let center = numbers.len() / 2;
    (numbers[..center].to_vec(), numbers[center..].to_vec())
}

fn add(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    if left.len() == right.len() {
        for (x, y) in zip(left, right) {
            result.push(x + y);
        }
    } else {
        result.push(right[0]);
        for i in 0..left.len() {
            result.push(left[i] + right[i + 1]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::split_add;

    #[test]
    fn test_split_add() {
        let test_cases = vec![
            (vec![1, 2, 3, 4, 5], 2, vec![5, 10]),
            (vec![1, 2, 3, 4, 5], 3, vec![15]),
            (vec![15], 3, vec![15]),
            (vec![1, 2, 3, 4], 1, vec![4, 6]),
            (vec![1, 2, 3, 4, 5, 6], 20, vec![21]),
            (vec![32, 45, 43, 23, 54, 23, 54, 34], 2, vec![183, 125]),
            (
                vec![32, 45, 43, 23, 54, 23, 54, 34],
                0,
                vec![32, 45, 43, 23, 54, 23, 54, 34],
            ),
            (
                vec![3, 234, 25, 345, 45, 34, 234, 235, 345],
                3,
                vec![305, 1195],
            ),
            (
                vec![
                    3, 234, 25, 345, 45, 34, 234, 235, 345, 34, 534, 45, 645, 645, 645, 4656, 45, 3,
                ],
                4,
                vec![1040, 7712],
            ),
            (
                vec![23, 345, 345, 345, 34536, 567, 568, 6, 34536, 54, 7546, 456],
                20,
                vec![79327],
            ),
        ];
        for test_case in test_cases {
            let numbers = test_case.0;
            let repeat = test_case.1;
            let want = test_case.2;
            let got = split_add(&numbers, repeat);
            assert_eq!(want, got);
        }
    }
}
