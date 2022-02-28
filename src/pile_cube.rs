fn find_nb(m: u64) -> i32 {
    let mut n = 1;
    loop {
        let s = calc(n);
        if s == m {
            return n as i32;
        } else if s > m {
            return -1;
        }
        n += 1
    }
}

fn calc(n:u64) -> u64 {
    let x = n * (n + 1) / 2;
    x * x
}

fn testing(n: u64, exp: i32) -> () {
    assert_eq!(find_nb(n), exp);
}

#[test]
fn basics_find_nb() {
    testing(4183059834009, 2022);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}
