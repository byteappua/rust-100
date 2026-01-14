# Day 17: 迭代器 (Iterators)

迭代器模式允许你对一个项的序列进行某些处理。迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。

## 1. Iterator Trait

在 Rust 中，迭代器都实现了 `Iterator` trait。

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 此处省略了其它有默认实现的方法
}
```

## 2. 使用迭代器

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

## 3. 消费适配器 (Consuming Adaptors)

调用 `next` 方法的方法被称为消费适配器，因为它们会消耗迭代器。

*   `sum()`: 获取所有权并反复调用 `next` 遍历迭代器，将结果求和。
*   `collect()`: 将迭代器转换成一个集合。

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
let total: i32 = v1_iter.sum();
```

## 4. 迭代器适配器 (Iterator Adaptors)

允许我们将当前迭代器转换成不同类型的迭代器。

*   `map()`: 对每个元素调用闭包。
*   `filter()`: 过滤元素。

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

## 5. 自定义迭代器

我们可以为自己的类型实现 `Iterator` trait。

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```
