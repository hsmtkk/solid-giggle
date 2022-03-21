fn points(games:&[&str]) -> u32 {
    let ps: Vec<u32> = games.iter().map(calc_point).collect();
    ps.iter().sum()
}

fn calc_point(s:&&str) -> u32 {
    let elems: Vec<&str> = s.split(":").collect();
    let x: u32 = elems[0].parse().unwrap();
    let y: u32 = elems[1].parse().unwrap();
    if x > y {
        3
    } else if x == y {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::points;

    #[test]
    fn test0(){
        let games = vec!["1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3"];
        assert_eq!(30, points(&games));
    }

    #[test]
    fn test1(){
        let games = vec!["1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4"];
        assert_eq!(10, points(&games));
    }

    #[test]
    fn test2(){
        let games = vec!["0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4"];
        assert_eq!(0, points(&games));
    }

    #[test]
    fn test3(){
        let games = vec!["1:0", "2:0", "3:0", "4:0", "2:1", "1:3", "1:4", "2:3", "2:4", "3:4"];
        assert_eq!(15, points(&games));
    }

    #[test]
    fn test4(){
        let games = vec!["1:0", "2:0", "3:0", "4:4", "2:2", "3:3", "1:4", "2:3", "2:4", "3:4"];
        assert_eq!(12, points(&games));
    }
}
