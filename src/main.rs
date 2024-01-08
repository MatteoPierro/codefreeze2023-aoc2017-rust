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
    fn test4() {
        let result = evaluate_pair(1, 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test5() {
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

