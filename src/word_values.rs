fn word_values(words:&[&str]) -> Vec<u32> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::word_values;

    #[test]
    fn test_word_values(){
        let test_cases = vec![
            (vec!["abc", "abc", "abc", "abc"], vec![6, 12, 18, 24]),
            (vec!["codewars", "abc", "xyz"], vec![88, 12, 225]),
        ];
        for (input,want) in test_cases {
            let got = word_values(&input);
            assert_eq!(want, got);
        }
    }
}

/*
		{[]string{}, []int{}},
		{[]string{}, []int{}},
*/