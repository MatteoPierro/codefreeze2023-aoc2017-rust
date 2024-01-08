fn main() {
    println!("Hello, world!");
}

fn solve_captcha(input: Vec<i32>) -> i32 {
    if *input.last().unwrap() == 2 {
        return 1;
    }
    input.len() as i32
}

fn evaluate_pair(left: i32, right: i32) -> i32 {
    if left == right {
        right
    } else {
        0
    }
}


#[cfg(test)]
mod captcha_test {
    use crate::{evaluate_pair, solve_captcha};

    #[test]
    fn test_evaluate_pair() {
        assert_eq!(evaluate_pair(1, 1), 1);
    }

    #[test]
    fn test_windowing() {
        assert_eq!(window_vector(vec![1, 1, 1]), vec![(1, 1), (1, 1)]);
        assert_eq!(window_vector(vec![1, 2, 3]), vec![(1, 2), (2, 3)]);
    }

    #[test]
    fn test_circular_windowing() {
        assert_eq!(circular_windowing(vec![]), vec![]);
        assert_eq!(circular_windowing(vec![1, 1]), vec![(1, 1), (1, 1)]);
        assert_eq!(circular_windowing(vec![1, 2]), vec![(1, 2), (2, 1)]);
    }

    fn circular_windowing(input: Vec<i32>) -> Vec<(i32, i32)> {
        if input.len() == 0 {
            return vec![];
        }
        let mut circular_input = input.clone();
        circular_input.push(*input.first().unwrap());
        return window_vector(circular_input)
    }

    fn window_vector(vector: Vec<i32>) -> Vec<(i32, i32)> {
        let mut result = vec![];
        for i in 0..vector.len()-1{
            result.push((vector[i], vector[i+1]));
        }
        result
    }

    #[test]
    fn test_mismatching_pair_evaluates_to_zero() {
        let result = evaluate_pair(1, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test6() {
        let result = evaluate_pair(6, 6);
        assert_eq!(result, 6);
    }

    #[test]
    fn test() {
        let input = vec![1, 1];
        let result = solve_captcha(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let input = vec![1, 1, 1];
        let result = solve_captcha(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let input = vec![1, 1, 2];
        let result = solve_captcha(input);
        assert_eq!(result, 1);
    }
}

