fn solve(v: &Vec<String>) -> i32 {
    let evens: i32 = v.iter().filter(is_even).count().try_into().unwrap();
    let odds: i32 = v.iter().filter(is_odd).count().try_into().unwrap();
    evens - odds
}

fn is_even(s: &&String) -> bool {
    match s.parse::<i32>() {
        Ok(n) => n % 2 == 0,
        Err(_) => false,
    }
}

fn is_odd(s: &&String) -> bool {
    match s.parse::<i32>() {
        Ok(n) => n % 2 == 1,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_tests() {
        let example_inputs = [
            &to_vector(&["0", "1", "2", "3"]),
            &to_vector(&["0", "1", "2", "3", "a", "b"]),
            &to_vector(&[
                "0", "15", "z", "16", "m", "13", "14", "c", "9", "10", "13", "u", "4", "3",
            ]),
            &to_vector(&[
                "5", "15", "16", "10", "6", "4", "16", "t", "13", "n", "14", "k", "n", "0", "q",
                "d", "7", "9",
            ]),
        ];

        assert_eq!(solve(example_inputs[0]), 0);
        assert_eq!(solve(example_inputs[1]), 0);
        assert_eq!(solve(example_inputs[2]), 0);
        assert_eq!(solve(example_inputs[3]), 2);
    }

    fn to_vector(strs: &[&'static str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }
}
