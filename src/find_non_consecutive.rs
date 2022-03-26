fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    let mut current = arr[0];
    for n in &arr[1..] {
        if current + 1 == *n {
            current = *n;
        } else {
            return Some(*n);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::first_non_consecutive;

    #[test]
    fn test_basic() {
        assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 6, 7, 8]), Some(6));
        assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 5, 6, 7, 8]), None);
        assert_eq!(first_non_consecutive(&vec![4, 6, 7, 8, 9, 11]), Some(6));
        assert_eq!(first_non_consecutive(&vec![4, 5, 6, 7, 8, 9, 11]), Some(11));
        assert_eq!(first_non_consecutive(&vec![31, 32]), None);
        assert_eq!(first_non_consecutive(&vec![-3, -2, 0, 1]), Some(0));
        assert_eq!(first_non_consecutive(&vec![-5, -4, -3, -1]), Some(-1));
    }
}
