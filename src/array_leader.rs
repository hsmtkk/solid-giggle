fn array_leaders(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..arr.len()-1{
        let sum = right_sum(arr, i);
        if arr[i] > sum {
            result.push(arr[i]);
        }
    }
    let last = arr[arr.len()-1];
    if last > 0 {
        result.push(last);
    }
    result
}

fn right_sum(arr:&[i32], i:usize) -> i32 {
    arr[i+1..].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Positive values
        assert_eq!(array_leaders(&[1,2,3,4,0]), [4]);
        assert_eq!(array_leaders(&[16,17,4,3,5,2]), [17,5,2]);

        // Negative values
        assert_eq!(array_leaders(&[-1,-29,-26,-2]), [-1]);
        assert_eq!(array_leaders(&[-36,-12,-27]),  [-36,-12]);

        // Mixed values
        assert_eq!(array_leaders(&[5,-2,2]), [5,2]);
        assert_eq!(array_leaders(&[0,-1,-29,3,2]),  [0,-1,3,2]);
    }
}
