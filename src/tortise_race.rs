use anyhow::{bail, Result};

#[derive(PartialEq, Debug)]
struct Time {
    hour: u32,
    minute: u32,
    second: u32,
}

impl Time {
    fn new(hour: u32, minute: u32, second: u32) -> Self {
        Time {
            hour,
            minute,
            second,
        }
    }
}

fn race(v1: u32, v2: u32, gap: u32) -> Result<Time> {
    if v1 >= v2 {
        bail!("v2 must be greater than v1");
    }
    let total_sec = 3600 * gap / (v2 - v1);
    let total_min = total_sec / 60;
    let sec = total_sec % 60;
    let hour = total_min / 60;
    let min = total_min % 60;
    Ok(Time::new(hour, min, sec))
}

#[cfg(test)]
mod tests {
    use super::{race, Time};

    #[test]
    fn test0() {
        let want = Time::new(0, 32, 18);
        let got = race(720, 850, 70).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn test1() {
        race(820, 81, 550).unwrap();
    }

    #[test]
    fn test2() {
        let want = Time::new(3, 21, 49);
        let got = race(80, 91, 37).unwrap();
        assert_eq!(want, got);
    }
}
