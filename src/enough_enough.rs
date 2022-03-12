use std::collections::HashMap;

fn delete_nth(lst: &[u8], threshold: usize) -> Vec<u8> {
    let mut number_counter: HashMap<u8, usize> = HashMap::new();
    let mut result: Vec<u8> = Vec::new();
    for n in lst {
        match number_counter.get(n){
            Some(count) => {
                number_counter.insert(*n, count+1);
            },
            None => {
                number_counter.insert(*n, 1);
            },
        }
        if let Some(count) = number_counter.get(n) {
            if *count <= threshold {
                result.push(*n);
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20,37,20,21], 1), vec![20,37,21]);
        assert_eq!(delete_nth(&[1,1,3,3,7,2,2,2,2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }
}