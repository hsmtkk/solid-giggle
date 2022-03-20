fn new_avg(fs: &[f32], target: i32) -> Option<u32> {
    let n: i32 = (fs.len() + 1) as i32;
    let sum: f32 = fs.into_iter().sum();
    let x = n * target - sum as i32;
    if x < 0 {
        return None;
    }
    Some(x.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::new_avg;

    #[test]
    fn test0() {
        let fs = vec![14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 16.0];
        assert_eq!(Some(628), new_avg(&fs, 90));
    }

    #[test]
    fn test1() {
        let fs = vec![14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
        assert_eq!(Some(645), new_avg(&fs, 92));
    }

    #[test]
    fn test2() {
        let fs = vec![1400.25, 30000.76, 5.56, 7.0, 9.0, 11.0, 15.48, 120.98];
        assert_eq!(None, new_avg(&fs, 2000));
    }
}
