struct Calculator;

impl Calculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Calculator::add(2, 2), 4);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(Calculator::subtract(5, 3), 2);
    }
}
