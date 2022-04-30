use std::collections::BTreeMap;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    for ch in input.to_lowercase().chars() {
        if ch.is_alphabetic() {
            result.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    result
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::letter_frequency;
    use std::collections::BTreeMap;

    #[test]
    fn simpleword() {
        let answer: BTreeMap<char, i32> = [('a', 2), ('c', 1), ('l', 1), ('t', 1), ('u', 1)]
            .iter()
            .cloned()
            .collect();

        assert_eq!(letter_frequency("actual"), answer);
    }

    #[test]
    fn sequence() {
        let answer: BTreeMap<char, i32> = [
            ('a', 3),
            ('b', 2),
            ('f', 1),
            ('p', 1),
            ('s', 1),
            ('t', 2),
            ('u', 1),
            ('x', 5),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(letter_frequency("AaabBF UttsP xxxxx"), answer);
    }
}
