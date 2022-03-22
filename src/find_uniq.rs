use std::collections::HashMap;

fn find_uniq(fs: &[f32]) -> f32 {
    let ds: Vec<String> = fs.iter().map(|f| f.to_string()).collect();
    let mut counter = HashMap::new();
    for d in ds {
        let opt_count = counter.get(&d);
        match opt_count {
            Some(count) => {
                counter.insert(d, count + 1);
            }
            None => {
                counter.insert(d, 1);
            }
        }
    }
    for (d, count) in counter {
        if count == 1 {
            let f: f32 = d.parse().unwrap();
            return f;
        }
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::find_uniq;

    #[test]
    fn test0() {
        let fs = vec![1.0, 1.0, 1.0, 2.0, 1.0, 1.0];
        assert_eq!(2.0, find_uniq(&fs))
    }

    #[test]
    fn test1() {
        let fs = vec![0.0, 0.0, 0.55, 0.0, 0.0];
        assert_eq!(0.55, find_uniq(&fs));
    }
}
