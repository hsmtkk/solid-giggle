use chrono::prelude::Utc;
use chrono::Duration;
use chrono::TimeZone;

fn date_nb_days(init: u32, target: u32, interest: u32) -> String {
    let daily_interest = interest as f64 / 36000.0;
    let mut days = 0;
    let mut current = init as f64;
    loop {
        current *= 1.0 + daily_interest;
        days += 1;
        if current >= target as f64 {
            break;
        }
    }
    let start = Utc.ymd(2016, 1, 1);
    let end = start + Duration::days(days);
    end.format("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        let want = "2024-07-03";
        let got = super::date_nb_days(4281, 5087, 2);
        assert_eq!(want, got);
    }

    #[test]
    fn test1() {
        let want = "2021-09-19";
        let got = super::date_nb_days(4620, 5188, 2);
        assert_eq!(want, got);
    }
}
