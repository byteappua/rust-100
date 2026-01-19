fn verify_percentage(x: f64) {
    if x < 0.0 || x > 1.0 {
        panic!("Percentage must be between 0.0 and 1.0");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: Add attribute to expect a panic
    fn test_out_of_bounds() {
        verify_percentage(2.5); // This should panic
    }
}
