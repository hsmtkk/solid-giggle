fn sum_of_n(n: i32) -> Vec<i32> {
    let m = if n < 0 {
        -n
    } else {
        n
    };
    let mut sum = 0;
    let mut ns = Vec::new();
    for i in 0..m+1 {
        sum += i;
        ns.push(sum);
    }
    if n < 0 {
        ns.iter().map(|n| -n).collect()
    } else {
        ns
    }
}

#[test]
fn normal_tests() {
    assert_eq!(sum_of_n(3), vec![0, 1, 3, 6]);
    assert_eq!(sum_of_n(-4), vec![0, -1, -3, -6, -10]);
    assert_eq!(sum_of_n(1), vec![0, 1]);
    assert_eq!(sum_of_n(0), vec![0]);
    assert_eq!(sum_of_n(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
}
