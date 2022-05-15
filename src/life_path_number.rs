fn life_path_number(s: &str) -> u32 {
    let splitted: Vec<&str> = s.split('-').collect();
    let year: u32 = splitted[0].parse().unwrap();
    let month: u32 = splitted[1].parse().unwrap();
    let day: u32 = splitted[2].parse().unwrap();
    compress_number(compress_number(year) + compress_number(month) + compress_number(day))
}

fn compress_number(num: u32) -> u32 {
    if num < 10 {
        return num;
    }
    let mut s = 0;
    let mut n = num;
    while n > 0 {
        s += n % 10;
        n /= 10;
    }
    compress_number(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Life Path Number - Einstein
        assert_eq!(life_path_number("1879-03-14"), 6);

        // Life Path Number - Ada Lovelace
        assert_eq!(life_path_number("1815-12-10"), 1);

        // Life Path Number - Brendan Eich
        assert_eq!(life_path_number("1961-07-04"), 1);

        // Life Path Number - Bill Gates
        assert_eq!(life_path_number("1955-10-28"), 4);

        // Life Path Number - Leonardo da Vinci
        assert_eq!(life_path_number("1452-04-15"), 4);

        // Life Path Number - Charles Babbage
        assert_eq!(life_path_number("1791-12-26"), 2);

        // Life Path Number - Grace Hopper
        assert_eq!(life_path_number("1906-12-09"), 1);

        // Life Path Number - Alan Turing
        assert_eq!(life_path_number("1912-06-23"), 6);

        // Life Path Number - Steve Wozniak
        assert_eq!(life_path_number("1950-08-11"), 7);

        // Life Path Number - Guido van Rossum
        assert_eq!(life_path_number("1956-01-31"), 8);

        // Life Path Number - Yukihiro Matsumoto
        assert_eq!(life_path_number("1965-04-14"), 3);

        // Life Path Number - Elon Musk
        assert_eq!(life_path_number("1971-06-28"), 7);
    }
}
