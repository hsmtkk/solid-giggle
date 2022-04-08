fn grid_index(grid: &[Vec<char>], indices: &[usize]) -> String {
    let size = grid.len();
    let mut cs: Vec<char> = Vec::new();
    for i in indices {
        let j = i - 1;
        let row = j / size;
        let column = j % size;
        cs.push(grid[row][column]);
    }
    cs.iter().collect()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::grid_index;

    #[test]
    fn sample_tests() {
        let grid1 = &vec![
            vec!['m', 'y', 'e'],
            vec!['x', 'a', 'm'],
            vec!['p', 'l', 'e'],
        ];

        assert_eq!(
            grid_index(grid1, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            "myexample"
        );
        assert_eq!(grid_index(grid1, &vec![1, 5, 6]), "mam");
        assert_eq!(grid_index(grid1, &vec![1, 3, 7, 8]), "mepl");

        let grid2 = &vec![
            vec!['h', 'e', 'l', 'l'],
            vec!['o', 'c', 'o', 'd'],
            vec!['e', 'w', 'a', 'r'],
            vec!['r', 'i', 'o', 'r'],
        ];

        assert_eq!(
            grid_index(grid2, &vec![5, 7, 9, 3, 6, 6, 8, 8, 16, 13]),
            "ooelccddrr"
        );
    }
}
