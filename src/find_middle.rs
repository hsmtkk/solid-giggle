fn gimme(input_array: [i32;3]) -> usize {
    let mut sorted = input_array.to_vec();
    sorted.sort();
    let middle = sorted[1];
    for i in 0..input_array.len(){
        if middle == input_array[i] {
            return i;
        }
    }
    0
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
