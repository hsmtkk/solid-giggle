fn encode(msg: String, n: i32) -> Vec<i32> {
    let mut nums = str_to_nums(&msg);
    let keys = int_to_nums(n);
    let key_len = keys.len();
    for i in 0..nums.len(){
        let j = i % key_len;
        nums[i] += keys[j];
    }
    nums
}

fn str_to_nums(msg:&str) -> Vec<i32> {
    let chs: Vec<char> = msg.chars().collect();
    chs.iter().map(char_to_num).collect()
}

fn char_to_num(ch:&char) -> i32 {
    match ch {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => 0,
    }
}

fn int_to_nums(n:i32) -> Vec<i32> {
    let digits: Vec<char> = n.to_string().chars().collect();
    digits.iter().map(digit_to_num).collect()
}

fn digit_to_num(ch:&char) -> i32 {
    match ch {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
        assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
    }
}
