use std::collections::HashMap;

fn repeats(arr: &Vec<i32>) -> i32 {
    let mut counter = HashMap::new();
    for n in arr {
        counter.entry(n).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut sum = 0;
    for (n, count) in counter {
        if count == 1 {
            sum += n;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::repeats;

    #[test]
    fn basic_tests() {
        assert_eq!(repeats(&vec![4, 5, 7, 5, 4, 8]), 15);
        assert_eq!(repeats(&vec![9, 10, 19, 13, 19, 13]), 19);
        assert_eq!(repeats(&vec![16, 0, 11, 4, 8, 16, 0, 11]), 12);
        assert_eq!(repeats(&vec![5, 17, 18, 11, 13, 18, 11, 13]), 22);
        assert_eq!(repeats(&vec![5, 10, 19, 13, 10, 13]), 24);
    }
}
