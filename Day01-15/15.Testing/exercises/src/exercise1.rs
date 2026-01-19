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
        // TODO: assert that 2 + 2 equals 4
    }

    #[test]
    fn test_subtract() {
        // TODO: assert that 5 - 3 equals 2
    }
}
