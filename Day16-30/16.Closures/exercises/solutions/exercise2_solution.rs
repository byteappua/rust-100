fn main() {
    let mut count = 0;

    // `inc` 借用了 `count` 的可变引用，因此闭包本身也必须是 `mut` 的
    // 并且它实现了 `FnMut` trait
    let mut inc = || {
        count += 1;
        count
    };

    println!("Count: {}", inc());
    println!("Count: {}", inc());
    println!("Count: {}", inc());

    assert_eq!(count, 3);
}
