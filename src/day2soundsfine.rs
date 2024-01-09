
fn max_value(input: &Vec<i32>) -> i32 {
    *input.iter().max().unwrap()
}
fn min_value(input: &Vec<i32>) -> i32 {
    *input.iter().min().unwrap()
}

#[cfg(test)]
mod checksum_corruption_test {
    use crate::*;
    use crate::day2soundsfine::{max_value, min_value};
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn test_max_value() {
        assert_eq!(9,max_value(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }
    #[test]
    fn test_calculate_checksum() {
        assert_eq!(8, calculate_row_checksum(&vec![5, 1, 9, 5]));
    }

    #[test]
    fn test_something() {
        assert_eq!(12, sum_rows_checksum(vec![vec![5,1,9,5], vec![7, 5, 3]]));
        assert_eq!(18, sum_rows_checksum(vec![vec![5,1,9,5], vec![7, 5, 3], vec![2, 4, 6, 8]]));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(vec![1, 2, 3], parse_line("1\t2\t3"))
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(vec![vec![1],vec![1]], parse_lines(vec!["1", "1"]));
        assert_eq!(vec![vec![1,2,3],vec![4,5,6]], parse_lines(vec!["1\t2\t3", "4\t5\t6"]))
    }

    #[test]
    fn test_full_checksum() {
        let lines = read_lines(&read_input_file("day2.txt"));
        let rows = lines.iter()
            .map(|line| parse_line(line))
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(51833,sum_rows_checksum(rows))
    }

    fn parse_lines(input: Vec<&str>) -> Vec<Vec<i32>> {
        input.iter().map(|x| parse_line(x)).collect()
    }

    fn parse_line(input: &str) -> Vec<i32> {
        input.split_whitespace()
            .map(|cell| cell.parse::<i32>().unwrap())
            .collect()
    }

    fn sum_rows_checksum(input: Vec<Vec<i32>>) -> i32 {
        input.iter().map(|row| calculate_row_checksum(row)).sum()
    }

    fn calculate_row_checksum(input: &Vec<i32>) -> i32 {
        max_value(&input) - min_value(&input)
    }
}