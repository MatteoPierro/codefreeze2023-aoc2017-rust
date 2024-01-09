
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
        assert_eq!(8, calculate_checksum(vec![5,1,9,5]));
    }

    fn calculate_checksum(input: Vec<i32>) -> i32 {
        max_value(&input) - min_value(&input)
    }
}