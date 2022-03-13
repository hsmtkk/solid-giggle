fn high_and_low(input:&str) -> String {
    let nums : Vec<i32> = input.split(" ").map(str_to_int).collect();
    let (high, low) = find_high_low(&nums);
    format!("{} {}", high, low)
}

fn str_to_int(s:&str) -> i32 {
    s.parse().unwrap()
}

fn find_high_low(nums: &[i32]) -> (i32, i32) {
    let mut high = nums[0];
    let mut low = nums[0];
    for n in nums {
        if *n > high {
            high = *n;
        }
        if *n < low {
            low = *n;
        }
    }
    (high, low)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0(){
        let input = "8 3 -5 42 -1 0 0 -9 4 7 4 -4";
        let got = super::high_and_low(input);
        let want = "42 -9";
        assert_eq!(want, got);
    }

    #[test]
    fn test1(){
        let input = "1 2 3";
        let got = super::high_and_low(input);
        let want = "3 1";
        assert_eq!(want, got);
    }

    #[test]
    fn test42(){
        let input = "42";
        let got = super::high_and_low(input);
        let want = "42 42";
        assert_eq!(want, got);
    }
}
