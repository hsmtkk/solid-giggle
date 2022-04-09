fn sum_or_product(list: &[i64], n: usize) -> String {
    let mut sorted = list.to_vec();
    sorted.sort();
    let mins = &sorted[..n];
    let maxs = &sorted[list.len()-n..];
    let sum: i64 = maxs.into_iter().sum();
    let pro: i64 = mins.into_iter().product();
    if sum == pro {
        "same"
    } else if sum > pro {
        "sum"
    } else {
        "product"
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sum_or_product(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), "sum");
        assert_eq!(sum_or_product(&[10, 41, 8, 16, 20, 36, 9, 13, 20], 3), "product");
        assert_eq!(sum_or_product(&[10, 20, 3, 30, 5, 4], 3), "same");
    }
}
