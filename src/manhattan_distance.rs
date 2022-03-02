fn manhattan_distance(point_a:&[u32], point_b:&[u32]) -> u32 {
    let mut dist: u32 = 0;
    for i in 0..point_a.len(){
        let x = point_a[i];
        let y = point_b[i];
        dist += abs(x,y);
    }
    dist
}

fn abs(x:u32, y:u32) -> u32 {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

#[cfg(test)]
mod tests {
    use super::manhattan_distance;

    #[test]
    fn test0(){
        assert_eq!(0, manhattan_distance(&vec![1,1], &vec![1,1]));
    }

    #[test]
    fn test1(){
        assert_eq!(4, manhattan_distance(&vec![5,4], &vec![3,2]));
    }

    #[test]
    fn test2(){
        assert_eq!(3, manhattan_distance(&vec![1,1], &vec![0,3]));
    }
}