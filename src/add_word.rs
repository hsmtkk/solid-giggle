struct Arith {
    value: &'static str,
}

const ZERO: &str = "zero";
const ONE: &str = "one";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";
const TEN: &str = "ten";
const ELEVEN: &str = "eleven";

impl Arith {
    fn add(&self, number: &str) -> &'static str {
        let m = string_to_int(self.value);
        let n = string_to_int(number);
        int_to_string(m + n)
    }
}

fn string_to_int(s: &str) -> u8 {
    match s {
        ZERO => 0,
        ONE => 1,
        TWO => 2,
        THREE => 3,
        FOUR => 4,
        FIVE => 5,
        SIX => 6,
        SEVEN => 7,
        EIGHT => 8,
        NINE => 9,
        TEN => 10,
        _ => 0,
    }
}

fn int_to_string(n: u8) -> &'static str {
    match n {
        0 => ZERO,
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        10 => TEN,
        11 => ELEVEN,
        _ => ZERO,
    }
}

#[cfg(test)]
mod tests {
    use super::Arith;

    #[test]
    fn returns_expected() {
        let c = Arith { value: "three" };
        assert_eq!(c.add("seven"), "ten");
        assert_eq!(c.add("eight"), "eleven");
        assert_eq!(c.add("zero"), "three");
    }
}
