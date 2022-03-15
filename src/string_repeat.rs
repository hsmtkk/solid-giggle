fn repeat_str(src: &str, count: usize) -> String {
    let ss: Vec<&str> = std::iter::repeat(src).take(count).collect();
    ss.join("")
}

#[test]
fn example_tests() {
    assert_eq!(repeat_str("a", 4), "aaaa");
    assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
    assert_eq!(repeat_str("abc", 2), "abcabc");
}
