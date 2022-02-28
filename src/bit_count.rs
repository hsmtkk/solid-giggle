fn count_bits(n: i64) -> u32 {
    let bs = format!("{:b}", n);
    bs.chars().fold(0, count)
}

fn count(x: u32, ch: char) -> u32 {
    if ch == '1' {
        x + 1
    } else {
        x
    }
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}
