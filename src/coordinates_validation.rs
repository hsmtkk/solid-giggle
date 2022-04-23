fn validate(input:&str) -> bool {
    let elems: Vec<&str> = input.split_whitespace().collect();
    if elems.len() != 2 {
        return false;
    }
    if input.contains('e') || input.contains('E') {
        return false;
    }
    let lat_str = elems[0];
    let long_str = elems[1];
    let lat = match lat_str.parse::<f32>() {
        Ok(x) => x,
        Err(_) => {return false;},
    };
    let long = match long_str.parse::<f32>() {
        Ok(x) => x,
        Err(_) => {return false;},
    };
    if lat < -90.0 || lat > 90.0 {
        return false;
    }
    if long < -180.0 || lat > 180.0 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn test_valid(){
        let inputs = vec![
            "-23, 25",
            "4, -3",
            "24.53525235, 23.45235",
            "04, -23.234235",
            "43.91343345, 143",
        ];
        for input in inputs {
            assert!(validate(input));
        }
    }

    #[test]
    fn test_invalid(){
        let inputs = vec![
            "23.234, - 23.4234",
            "2342.43536, 34.324236",
            "N23.43345, E32.6457",
            "99.234, 12.324",
            "6.325624, 43.34345.345",
            "0, 1,2",
            "0.342q0832, 1.2324",
            "23.245, 1e1",
        ];
        for input in inputs {
            assert!(!validate(input));
        }
    }
}