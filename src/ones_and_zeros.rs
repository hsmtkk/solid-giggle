fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let mut s: u32 = 0;
    for i in 0..slice.len() {
        let p = slice.len()  - i - 1;
        if slice[i] > 0 {
            s += 2_u32.pow(p as u32);
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::binary_slice_to_number;

    #[test]
    fn example_tests() {
        assert_eq!(binary_slice_to_number(&vec![0,0,0,1]), 1);
        assert_eq!(binary_slice_to_number(&vec![0,0,1,0]), 2);
        assert_eq!(binary_slice_to_number(&vec![1,1,1,1]), 15);
        assert_eq!(binary_slice_to_number(&vec![0,1,1,0]), 6);
    }
}