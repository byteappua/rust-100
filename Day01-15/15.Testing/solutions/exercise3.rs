fn verify_percentage(x: f64) {
    if x < 0.0 || x > 1.0 {
        panic!("Percentage must be between 0.0 and 1.0");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Percentage must be between 0.0 and 1.0")]
    fn test_out_of_bounds() {
        verify_percentage(2.5);
    }
}
