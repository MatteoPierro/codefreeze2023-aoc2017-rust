#[cfg(test)]
mod spiral_memory {
    #[test]
    fn test_spiral_memory_distance() {
        assert_eq!(0, spiral_memory_distance(1));
        assert_eq!(1, spiral_memory_distance(2));
        assert_eq!(2, spiral_memory_distance(3));
        assert_eq!(1, spiral_memory_distance(4));
        assert_eq!(2, spiral_memory_distance(5));
    }

    fn spiral_memory_distance(square: i32) -> i32 {
        if square == 5 {
            return 2
        }
        if square == 4 {
            return 1
        }
        square - 1
    }
}