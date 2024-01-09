
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

    #[test]
    fn test_max_value() {
        assert_eq!(9,max_value(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }
    #[test]
    fn test_calculate_checksum() {
        assert_eq!(8, calculate_checksum(&vec![5,1,9,5]));
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

    fn parse_line(input: &str) -> Vec<i32> {
        let mut cell_iterator = input.split_whitespace();
        cell_iterator.map(|cell| cell.parse::<i32>().unwrap()).collect()
    }

    fn sum_rows_checksum(input: Vec<Vec<i32>>) -> i32 {
        input.iter().map(|row| calculate_checksum(row)).sum()
    }

    fn calculate_checksum(input: &Vec<i32>) -> i32 {
        max_value(&input) - min_value(&input)
    }
}