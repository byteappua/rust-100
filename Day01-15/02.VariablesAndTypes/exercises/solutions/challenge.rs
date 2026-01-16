fn main() {
    let numbers = [10, 20, 30, 40, 50];

    println!("First element: {}", numbers[0]);
    println!("Last element: {}", numbers[4]);

    // 手动计算和
    let sum = numbers[0] + numbers[1] + numbers[2] + numbers[3] + numbers[4];
    println!("Sum of elements: {}", sum);

    // 或者使用循环 (如果还没学到，可以作为拓展)
    let mut loop_sum = 0;
    for n in numbers.iter() {
        loop_sum += n;
    }
    println!("Sum (loop): {}", loop_sum);
}
