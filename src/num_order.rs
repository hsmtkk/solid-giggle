fn in_asc_order(nums:&[u32]) -> bool {
    let mut n = nums[0];
    for i in 1..nums.len(){
        if n > nums[i] {
            return false;
        }
        n = nums[i];
    }
    true
}

#[cfg(test)]
mod  tests {
    use super::in_asc_order;

    #[test]
    fn test0(){
        assert!(in_asc_order(&vec![1, 2, 4, 7, 19]));
    }

    #[test]
    fn test1(){
        assert!(in_asc_order(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test2(){
        assert!(!in_asc_order(&vec![1, 6, 10, 18, 2, 4, 20]));
    }

    #[test]
    fn test3(){
        assert!(!in_asc_order(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1]));
    }
}
