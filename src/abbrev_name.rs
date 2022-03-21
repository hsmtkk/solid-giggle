fn abbrev_name(name: &str) -> String {
    let elems: Vec<&str> = name.split(' ').collect();
    let first = elems[0].chars().next().unwrap().to_uppercase().to_string();
    let last = elems[1].chars().next().unwrap().to_uppercase().to_string();
    format!("{}.{}", first, last)
}

#[cfg(test)]
mod tests {
    use super::abbrev_name;
    use maplit::hashmap;

    #[test]
    fn test() {
        let name_want = hashmap! {
                    "Sam Harris" =>     "S.H",
        "Patrick Feenan"=> "P.F",
        "Evan Cole"=>      "E.C",
        "P Favuzzi"=>      "P.F",
        "David Mendieta"=> "D.M",
        };
        for (name, want) in name_want {
            let got = abbrev_name(name);
            assert_eq!(want, got);
        }
    }
}
