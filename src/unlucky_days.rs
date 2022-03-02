use chrono::{Datelike, TimeZone, Weekday};
use chrono::prelude::Utc;

fn unlucky_days(year:i32) -> u32 {
    let mut days = 0;
    for month in 1..13 {
        let dt = Utc.ymd(year, month, 13);
        if dt.weekday() == Weekday::Fri {
            days += 1;
        }
    }
    days
}

#[cfg(test)]
mod tests {
    use super::unlucky_days;

    #[test]
    fn test1986(){
        assert_eq!(1, unlucky_days(1986));
    }

    #[test]
    fn test2015(){
        assert_eq!(3, unlucky_days(2015));
    }
}