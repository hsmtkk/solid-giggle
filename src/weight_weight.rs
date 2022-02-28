use std::cmp::Ordering;

fn order_weight(s: &str) -> String {
    let mut weights: Vec<&str> = s.split(" ").collect();
    weights.sort_by(compare_weight);
    weights.join(" ")
}

fn compare_weight(s0: &&str, s1: &&str) -> Ordering {
    let w0 = calc_weight(s0);
    let w1 = calc_weight(s1);
    if w0 == w1 {
        s0.cmp(s1)
    } else {
        w0.cmp(&w1)
    }
}

fn char_to_digit(acc: u32, ch: char) -> u32 {
    let n = ch.to_digit(10).unwrap();
    acc + n
}

fn calc_weight(s: &str) -> u32 {
    s.chars().fold(0, char_to_digit)
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
