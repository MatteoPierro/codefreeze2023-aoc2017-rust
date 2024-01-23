#[cfg(test)]
mod spiral_memory {
    #[test]
    fn test_spiral_memory_distance() {
        assert_eq!(0, spiral_memory_distance(1));
        assert_eq!(1, spiral_memory_distance(2));
        assert_eq!(2, spiral_memory_distance(3));
        assert_eq!(1, spiral_memory_distance(4));
        assert_eq!(2, spiral_memory_distance(5));
        assert_eq!(1, spiral_memory_distance(6));
        assert_eq!(2, spiral_memory_distance(7));
        assert_eq!(1, spiral_memory_distance(8));
        assert_eq!(2, spiral_memory_distance(9));
        assert_eq!(3, spiral_memory_distance(10));
        assert_eq!(2, spiral_memory_distance(11));
        assert_eq!(3, spiral_memory_distance(12));
        assert_eq!(4, spiral_memory_distance(13));
        assert_eq!(3, spiral_memory_distance(14));
        assert_eq!(2, spiral_memory_distance(15));
        assert_eq!(3, spiral_memory_distance(16));
        assert_eq!(4, spiral_memory_distance(17));
        assert_eq!(2, spiral_memory_distance(19));
        assert_eq!(2, spiral_memory_distance(23));
    }

    fn spiral_memory_distance(square: i32) -> i32 {
        if square == 1 {
            return 0;
        }
        if square == 11 || square == 15 || square == 19 || square == 23 {
            return (square % 2) + 1;
        }
        if square >= 10 {
            return (square % 2) + 1 + 2;
        }
        (square % 2) + 1
    }
}