fn find_digit(num: i32, nth: i32) -> i32 {
    if nth <= 0 {
        return -1;
    }
    let digits = if num > 0 { num } else { -num }.to_string();
    let index = digits.len() as i32 - nth;
    if index < 0 {
        0
    } else {
        let d: char = digits.chars().nth(index as usize).unwrap();
        d.to_digit(10).unwrap() as i32
    }
}

#[test]
fn example_test() {
    assert_eq!(find_digit(5673, 4), 5);
    assert_eq!(find_digit(129, 2), 2);
    assert_eq!(find_digit(-2825, 3), 8);
    assert_eq!(find_digit(-456, 4), 0);
    assert_eq!(find_digit(0, 20), 0);
    assert_eq!(find_digit(65, 0), -1);
    assert_eq!(find_digit(24, -8), -1);
}
