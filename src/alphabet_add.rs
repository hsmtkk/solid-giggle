fn add_letters(letters: Vec<char>) -> char {
    if letters.is_empty(){
        return 'z';
    }
    let ns: Vec<u8> = letters.iter().map(char_to_u8).collect();
    let s: u8 = ns.iter().sum();
    u8_to_char(s % 26)
}

fn char_to_u8(ch:&char) -> u8 {
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

fn u8_to_char(n:u8) -> char {
    match n {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        9 => 'i',
        10=> 'j',
        11=> 'k',
        12=> 'l',
        13=> 'm',
        14=> 'n',
        15=> 'o',
        16=> 'p',
        17=> 'q',
        18=> 'r',
        19=> 's',
        20=> 't',
        21=> 'u',
        22=> 'v',
        23=> 'w',
        24=> 'x',
        25=> 'y',
        26 => 'z',
        _ => 'z',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(add_letters(vec!['a', 'b', 'c']), 'f');
        assert_eq!(add_letters(vec!['z']), 'z');
        assert_eq!(add_letters(vec!['a', 'b']), 'c');
        assert_eq!(add_letters(vec!['c']), 'c');
        assert_eq!(add_letters(vec!['z', 'a']), 'a');
        assert_eq!(add_letters(vec!['y', 'c', 'b']), 'd');
        assert_eq!(add_letters(vec![]), 'z');
    }
}