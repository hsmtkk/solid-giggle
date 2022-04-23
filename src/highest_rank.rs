use std::collections::HashMap;

fn highest_rank(nums: &[i32]) -> usize {
    let mut counter: HashMap<i32, usize> = HashMap::new();
    for n in nums {
        let count = counter.entry(*n).or_insert(0);
        *count += 1;
    }
    let mut max_number = nums[0];
    let mut max_count = 0;
    for (number, count) in counter {
        if count > max_count {
            max_count = count;
            max_number = number;
        } else if count == max_count && number > max_number {
            max_number = number;
        }
    }
    max_count
}

#[cfg(test)]
mod tests {
    fn test_highest_rank() {
        let test_cases = vec![
            (vec![12, 10, 8, 12, 7, 6, 4, 10, 12], 12),
            (vec![12, 10, 8, 12, 7, 6, 4, 10, 12, 10], 12),
            (vec![12, 10, 8, 8, 3, 3, 3, 3, 2, 4, 10, 12, 10], 3),
        ];
        for test_case in test_cases {
            let input = test_case.0;
            let want = test_case.1;
            assert_eq!(want, super::highest_rank(&input));
        }
    }
}
