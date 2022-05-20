fn pairs(arr: &Vec<i32>) -> usize {
    split_pair(arr).into_iter().filter(is_consecutive).count()
}

fn split_pair(ns:&[i32]) -> Vec<(i32, i32)>{
    let mut result = Vec::new();
    for i in (0..ns.len()-1).step_by(2){
        result.push((ns[i], ns[i+1]))
    }
    result
}

fn is_consecutive(pair: &(i32, i32)) -> bool {
    let x = &pair.0;
    let y = &pair.1;
    (x + 1 == *y) || (x - 1 == *y)
}

#[cfg(test)]
mod tests {
    use super::pairs;

    #[test]
    fn example_tests() {
        assert_eq!(pairs(&vec![1, 2, 5, 8, -4, -3, 7, 6, 5]), 3);
        assert_eq!(pairs(&vec![21, 20, 22, 40, 39, -56, 30, -55, 95, 94]), 2);
        assert_eq!(pairs(&vec![81, 44, 80, 26, 12, 27, -34, 37, -35]), 0);
        assert_eq!(pairs(&vec![-55, -56, -7, -6, 56, 55, 63, 62]), 4);
        assert_eq!(pairs(&vec![73, 72, 8, 9, 73, 72]), 3);
    }
}
