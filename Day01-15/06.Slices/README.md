# Day 06: 切片 (Slices)

切片 (Slice) 允许你引用集合中一段连续的元素序列，而不需要引用整个集合。切片是一种引用，所以它没有所有权。

## 1. 字符串切片 (String Slices)

字符串切片是指向 `String` 中一部分内容的引用。

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

`[starting_index..ending_index]` 语法：
*   从 `starting_index` 开始（包含）。
*   到 `ending_index` 结束（不包含）。

### 简写

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..2];
let slice = &s[..2]; // 等价

let slice = &s[3..len];
let slice = &s[3..]; // 等价

let slice = &s[0..len];
let slice = &s[..]; // 等价
```

## 2. 字符串字面值就是切片

我们之前见过的：

```rust
let s = "Hello, world!";
```

这里的 `s` 的类型是 `&str`：它是一个指向二进制程序特定位置的切片。这也是为什么字符串字面值是不可变的。

## 3. 其他类型的切片

数组也可以有切片。

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

## 4. 练习

编写一个函数，接收一个字符串，返回它的第一个单词。如果字符串中没有空格，返回整个字符串。

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
