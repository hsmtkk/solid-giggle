struct Cipher {
    map1: Vec<char>,
    map2: Vec<char>,
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        Cipher{map1:map1.chars().collect(), map2:map2.chars().collect()}
    }

    fn encode(&self, string: &str) -> String {
        let mut encoded = Vec::new();
        for ch in string.chars(){
            encoded.push(self.encode_char(ch));
        }
        encoded.into_iter().collect()
    }

    fn encode_char(&self, ch:char) -> char {
        if let Some(i) = self.map1.iter().position(|c| *c == ch) {
            self.map2[i]
        } else {
            ch
        }
    }

    fn decode(&self, string: &str) -> String {
        let mut decoded = Vec::new();
        for ch in string.chars(){
            decoded.push(self.decode_char(ch));
        }
        decoded.into_iter().collect()
    }

    fn decode_char(&self, ch:char) -> char {
        if let Some(i) = self.map2.iter().position(|c| *c == ch) {
            self.map1[i]
        } else {
            ch
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cipher;

    #[test]
    fn examples() {
        let map1 = "abcdefghijklmnopqrstuvwxyz";
        let map2 = "etaoinshrdlucmfwypvbgkjqxz";

        let cipher = Cipher::new(map1, map2);

        assert_eq!(cipher.encode("abc"), "eta");
        assert_eq!(cipher.encode("xyz"), "qxz");
        assert_eq!(cipher.decode("eirfg"), "aeiou");
        assert_eq!(cipher.decode("erlang"), "aikcfu");
    }
}