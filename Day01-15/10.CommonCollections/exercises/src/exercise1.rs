use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 2, 2, 3, 5, 1, 2, 4, 9, 2, 5, 3];
    println!("Numbers: {:?}", numbers);

    let mean = calculate_mean(&numbers);
    let median = calculate_median(&numbers);
    let mode = calculate_mode(&numbers);

    println!("Mean: {:.2}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);

    // Verify
    assert!((mean - 3.25).abs() < 1e-10);
    assert_eq!(median, 2); // Sorted: 1, 1, 2, 2, 2, 2, 3, 3, 4, 5, 5, 9. Mid: 2, 3 -> 2.5? or just mid element.
                           // 12 elements. 6th and 7th are 2 and 3. (2+3)/2 = 2.5? Or if integer, usually just one.
                           // Let's assume simplest definition for int vector: the middle element.
                           // Or float? The exercise asks for "Median". Let's stick to returning f64 or int.
                           // The solution usually implies handling even count.
}

fn calculate_mean(numbers: &Vec<i32>) -> f64 {
    // TODO: Implement
    0.0
}

fn calculate_median(numbers: &Vec<i32>) -> f64 {
    // TODO: Implement. Hint: Sort the vector first (maybe clone it?)
    0.0
}

fn calculate_mode(numbers: &Vec<i32>) -> i32 {
    // TODO: Implement using HashMap
    0
}
