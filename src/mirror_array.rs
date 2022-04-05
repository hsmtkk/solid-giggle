fn mirror(list: &[i32]) -> Vec<i32> {
    if list.is_empty(){
        return vec![];
    }
    let mut left: Vec<i32> = list.to_vec();
    left.sort();
    let mut right = left.to_vec();
    right.reverse();
    right.remove(0);
    let mut result = Vec::new();
    for x in left {
        result.push(x);
    }
    for x in right {
        result.push(x);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(mirror(&[]), &[]);
        assert_eq!(mirror(&[1]), &[1]);
        assert_eq!(mirror(&[2, 1]), &[1, 2, 1]);
        assert_eq!(mirror(&[2, 3, 1]), &[1, 2, 3, 2, 1]);
        assert_eq!(mirror(&[-8, 42, 18, 0, -16]), &[-16, -8, 0, 18, 42, 18, 0, -8, -16]);
    }
}
