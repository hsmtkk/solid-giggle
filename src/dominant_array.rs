fn solve(arr: &[u32]) -> Vec<u32> {
    let mut result = Vec::new();
    for i in 0..arr.len() - 1 {
        if is_greater_than(arr[i], &arr[i + 1..]) {
            result.push(arr[i]);
        }
    }
    result.push(arr[arr.len() - 1]);
    result
}

fn is_greater_than(n: u32, ns: &[u32]) -> bool {
    for m in ns {
        if n <= *m {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve(&[16, 17, 14, 3, 14, 5, 2]), [17, 14, 5, 2]);
        assert_eq!(solve(&[92, 52, 93, 31, 89, 87, 77, 105]), [105]);
        assert_eq!(solve(&[75, 47, 42, 56, 13, 55]), [75, 56, 55]);
        assert_eq!(solve(&[67, 54, 27, 85, 66, 88, 31, 24, 49]), [88, 49]);
        assert_eq!(solve(&[76, 17, 25, 36, 29]), [76, 36, 29]);
        assert_eq!(solve(&[104, 18, 37, 9, 36, 47, 28]), [104, 47, 28]);
    }
}
