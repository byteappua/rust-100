fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let result: i32 = numbers.iter()
        .filter(|&x| x % 2 == 0) // Filter evens
        .map(|x| x * x)          // Square them
        .sum();                  // Sum them

    println!("Result: {}", result);
    assert_eq!(result, 56);
}
