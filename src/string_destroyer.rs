use std::collections::HashSet;

fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let alphabets = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let filtered: Vec<String> = alphabets
        .into_iter()
        .map(|ch| convert_underscore(&input_sets, &ch))
        .collect();
    filtered.join(" ")
}

fn convert_underscore(input_sets: &Vec<HashSet<char>>, ch: &char) -> String {
    for set in input_sets {
        if set.contains(ch) {
            return "_".to_string();
        }
    }
    ch.to_string()
}

#[cfg(test)]
mod tests {
    use super::destroy;
    use std::collections::HashSet;

    #[test]
    fn basic_test1() {
        let mut input_set: Vec<HashSet<char>> = Vec::new();
        input_set.push(['A', 'b'].iter().cloned().collect());
        input_set.push(['C', 'd'].iter().cloned().collect());
        assert_eq!(
            destroy(input_set),
            "a _ c _ e f g h i j k l m n o p q r s t u v w x y z"
        );
    }

    #[test]
    fn basic_test2() {
        let mut input_set: Vec<HashSet<char>> = Vec::new();
        input_set.push(['B', 'b'].iter().cloned().collect());
        input_set.push(['C', 'm', 'f'].iter().cloned().collect());
        assert_eq!(
            destroy(input_set),
            "a _ c d e _ g h i j k l _ n o p q r s t u v w x y z"
        );
    }
}
