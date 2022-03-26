use std::collections::HashMap;

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut squared = HashMap::new();
    for n in a {
        squared.insert(n*n, true);
    }
    for n in b {
        match squared.get(&n) {
            Some(_) => {},
            None => {return false;},
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::comp;

    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        assert_eq!(comp(a, b), exp)
    }

    #[test]
    fn tests_comp() {
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, false);
    }
}

