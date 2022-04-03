use std::collections::HashMap;

fn doubleton(num: u32) -> u32 {
    let mut n = num + 1;
    loop {
        if is_doubleton(n) {
            return n;
        }
        n += 1;
    }
}

fn is_doubleton(n: u32) -> bool {
    let mut digit_counter = HashMap::new();
    for ch in n.to_string().chars() {
        match digit_counter.get(&ch) {
            Some(count) => {
                digit_counter.insert(ch, count + 1);
            }
            None => {
                digit_counter.insert(ch, 1);
            }
        }
    }
    digit_counter.keys().count() == 2
}

// Rust translation by B1ts 2021-03
// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::doubleton;

    #[test]
    fn sample_tests() {
        assert_eq!(doubleton(1), 10);
        assert_eq!(doubleton(10), 12);
        assert_eq!(doubleton(120), 121);
        assert_eq!(doubleton(1234), 1311);
    }
}
