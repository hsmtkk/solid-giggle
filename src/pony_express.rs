fn riders(stations: &Vec<u32>) -> u32 {
    let mut riders = 1;
    let mut dist = 0;
    for s in stations {
        if dist + s > 100 {
            riders += 1;
            dist = 0;
        }
        dist += s;
    }
    riders
}

#[test]
fn sample_tests() {
    assert_eq!(riders(&vec![18, 15]), 1);
    assert_eq!(riders(&vec![43, 23, 40, 13]), 2);
    assert_eq!(riders(&vec![33, 8, 16, 47, 30, 30, 46]), 3);
    assert_eq!(riders(&vec![6, 24, 6, 8, 28, 8, 23, 47, 17, 29, 37, 18, 40, 49]), 4);
}
