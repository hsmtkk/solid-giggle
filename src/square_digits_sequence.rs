fn square_digits_sequence(a0: u32) -> usize {
    let mut current = a0;
    let mut numbers = vec![a0];
    loop {
        current = next(current);
        if numbers.contains(&current) {
            break;
        }
        numbers.push(current);
    }
    numbers.len() + 1
}

fn next(n: u32) -> u32 {
    let mut sum = 0;
    for d in n.to_string().chars() {
        let m = d.to_digit(10).unwrap();
        sum += m * m;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(square_digits_sequence(16), 9);
        assert_eq!(square_digits_sequence(103), 4);
        assert_eq!(square_digits_sequence(1), 2);
    }
}
