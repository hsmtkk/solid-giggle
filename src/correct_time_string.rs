fn time_correct(time_str: &str) -> Option<String> {
    let splitted: Vec<&str> = time_str.split(':').collect();
    if splitted.len() != 3 {
        return None;
    }
    if let Ok(mut hour) = splitted[0].parse::<u32>(){
        if let Ok(mut min) = splitted[1].parse::<u32>(){
            if let Ok(mut sec) = splitted[2].parse::<u32>(){
                min += sec / 60;
                sec %= 60;
                hour += min / 60;
                min %= 60;
                hour %= 24;
                return Some(format!("{:02}:{:02}:{:02}", hour, min, sec));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(time_correct("").is_none());
    }

    #[test]
    fn invalid_format() {
        assert!(time_correct("001122").is_none());
        assert!(time_correct("00;11;22").is_none());
        assert!(time_correct("00:1c:22").is_none());
    }

    #[test]
    fn corrections() {
        assert_eq!(time_correct("09:10:01"), Some(String::from("09:10:01")));
        assert_eq!(time_correct("11:70:10"), Some(String::from("12:10:10")));
        assert_eq!(time_correct("19:99:09"), Some(String::from("20:39:09")));
        assert_eq!(time_correct("19:99:99"), Some(String::from("20:40:39")));
        assert_eq!(time_correct("24:01:01"), Some(String::from("00:01:01")));
        assert_eq!(time_correct("52:01:01"), Some(String::from("04:01:01")));
    }
}