fn compose_square_string(s1:&str, s2:&str) -> String {
    let ss1: Vec<&str> = s1.split("\n").collect();
    let ss2: Vec<&str> = s2.split("\n").collect();
    let size = ss1.len();
    let mut result: Vec<String> = Vec::new();
    for i in 0..size {
        let left = &ss1[i][..i+1];
        let right = &ss2[size-i-1][..size-i];
        let mut s = String::new();
        s += left;
        s += right;
        result.push(s);
    }
    result.join("\n")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0(){
        let got = super::compose_square_string("byGt\nhTts\nRTFF\nCnnI", "jIRl\nViBu\nrWOb\nNkTB");
        let want = "bNkTB\nhTrWO\nRTFVi\nCnnIj";
        assert_eq!(want,got);
    }

    #[test]
    fn test1(){
        let got = super::compose_square_string("HXxA\nTGBf\nIPhg\nuUMD", "Hcbj\nqteH\nGbMJ\ngYPW");
        let want = "HgYPW\nTGGbM\nIPhqt\nuUMDH";
        assert_eq!(want,got);
    }
}