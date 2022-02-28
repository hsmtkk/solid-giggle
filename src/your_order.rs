use std::cmp::Ordering;

fn order(sentence: &str) -> String {
    if sentence == "" {
        return "".to_string();
    }
    let mut word_indexes: Vec<WordIndex> = sentence.split(" ").map(|s| WordIndex::new(s)).collect();
    word_indexes.sort();
    let words: Vec<String> = word_indexes.iter().map(|wi| wi.to_string()).collect();
    words.join(" ")
}

struct WordIndex {
    word: String,
    index: usize,
}

impl WordIndex {
    fn new(word: &str) -> Self {
        let index = find_index(word);
        WordIndex {
            word: word.to_string(),
            index,
        }
    }
}

impl ToString for WordIndex {
    fn to_string(&self) -> String {
        self.word.clone()
    }
}

impl PartialEq for WordIndex {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl Eq for WordIndex {}

impl PartialOrd for WordIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for WordIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

fn find_index(word: &str) -> usize {
    for ch in word.chars() {
        let index = match ch {
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            _ => None,
        };
        if let Some(i) = index {
            return i;
        }
    }
    panic!("cannot find index");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
