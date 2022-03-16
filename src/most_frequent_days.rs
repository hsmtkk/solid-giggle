use chrono::prelude::*;
use std::collections::HashMap;
use std::ops::Add;

fn most_frequent_days(year:i32) -> Vec<Weekday> {
    let mut d = Utc.ymd(year, 1, 1).and_hms(1, 0, 0);
    let end = Utc.ymd(year+1, 1, 1).and_hms(0, 0, 0);
    let mut weekday_counter: HashMap<Weekday, u32> = HashMap::new();
    loop {
        let wd = d.weekday();
        match weekday_counter.get(&wd){
            Some(count) => {weekday_counter.insert(wd, count+1);},
            None => {weekday_counter.insert(wd, 1);},
        }
        d = d.add(chrono::Duration::days(1));
        if d >= end {
            break;
        }
    }
    let max = *weekday_counter.values().max().unwrap();
    let mut result = Vec::new();
    for (wd, count) in weekday_counter {
        if max == count {
            result.push(wd);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use chrono::Weekday;
    use super::most_frequent_days;

    #[test]
    fn test0(){
        assert_eq!(vec![Weekday::Fri], most_frequent_days(2427));
    }

    #[test]
    fn test1(){
        assert_eq!(vec![Weekday::Sat], most_frequent_days(2185));
    }

    #[test]
    fn test2(){
        assert_eq!(vec![Weekday::Mon], most_frequent_days(1770));
    }

    #[test]
    fn test3(){
        assert_eq!(vec![Weekday::Sat], most_frequent_days(1785));
    }

    #[test]
    fn test4(){
        assert_eq!(vec![Weekday::Mon, Weekday::Sun], most_frequent_days(1984));
    }

    #[test]
    fn test5(){
        assert_eq!(vec![Weekday::Sat, Weekday::Sun], most_frequent_days(3076));
    }
}
