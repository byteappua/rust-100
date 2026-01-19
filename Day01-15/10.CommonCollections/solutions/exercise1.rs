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
}

fn calculate_mean(numbers: &Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

fn calculate_median(numbers: &Vec<i32>) -> f64 {
    let mut sorted = numbers.clone();
    sorted.sort();

    let len = sorted.len();
    if len % 2 == 0 {
        let mid1 = sorted[len / 2 - 1];
        let mid2 = sorted[len / 2];
        (mid1 + mid2) as f64 / 2.0
    } else {
        sorted[len / 2] as f64
    }
}

fn calculate_mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &num in numbers {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;

    for (&num, &count) in &map {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }
    mode
}
