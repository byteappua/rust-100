# Day 10: 常用集合 (Common Collections)

Rust 标准库中包含了一系列被称为集合的数据结构。与数组和元组不同，这些集合指向的数据是存储在堆上的，这意味着数据的大小不需要在编译时确定，并且可以根据需要增长或缩小。

## 1. Vector (Vec<T>)

`Vec<T>` 也被称为 vector，允许我们在一个单独的数据结构中存储多个相同类型的值。

### 创建 Vector

```rust
let v: Vec<i32> = Vec::new(); // 创建空的 vector
let v = vec![1, 2, 3]; // 使用 vec! 宏创建
```

### 更新 Vector

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### 读取元素

有两种方式引用 vector 中的值：

1.  **索引语法**: 使用 `&v[index]`。如果索引越界，程序会 panic。
2.  **get 方法**: 使用 `v.get(index)`，返回 `Option<&T>`。如果索引越界，返回 `None`。

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

### 遍历 Vector

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// 遍历并修改
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50; // 使用解引用运算符 (*) 修改值
}
```

### 使用枚举存储多种类型

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## 2. String (字符串)

Rust 的核心语言层面只有一种字符串类型：字符串切片 `str` (通常以 `&str` 出现)。`String` 类型是由标准库提供的，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。

### 创建 String

```rust
let mut s = String::new();
let s = "initial contents".to_string();
let s = String::from("initial contents");
```

### 更新 String

```rust
let mut s = String::from("foo");
s.push_str("bar"); // 附加字符串切片
s.push('l'); // 附加单个字符
```

### 拼接 String

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // 注意 s1 被移动了，不能再被使用

// 使用 format! 宏 (不会获取所有权)
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
```

### 索引 String

**Rust 不支持字符串索引**。例如 `s[0]` 是非法的。
因为 String 是 UTF-8 编码的 wrapper，索引操作（O(1)）无法保证总是返回一个有效的字符（某些字符可能占用多个字节），且遍历 UTF-8 字符串以寻找第 N 个字符的代价是 O(N)。

### 切片 String

可以使用切片获取特定范围的字节，但必须谨慎，如果切片位置不在字符边界，程序会 panic。

```rust
let hello = "Здравствуйте";
let s = &hello[0..4]; // "Зд"
```

## 3. HashMap (哈希映射)

`HashMap<K, V>` 存储了类型为 `K` 的键和类型为 `V` 的值之间的映射关系。

### 创建 HashMap

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

### 访问值

```rust
let team_name = String::from("Blue");
let score = scores.get(&team_name); // 返回 Option<&i32>
```

### 遍历 HashMap

```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### 更新 HashMap

1.  **覆盖值**: 再次调用 `insert`。
2.  **只在键没有对应值时插入**: 使用 `entry`。

```rust
scores.entry(String::from("Blue")).or_insert(50);
scores.entry(String::from("Yellow")).or_insert(50);
```

3.  **根据旧值更新新值**:

```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

### 哈希函数

默认情况下，HashMap 使用 SipHash，这可以抵抗哈希表拒绝服务 (DoS) 攻击。如果需要更快的速度，可以指定其他的 hasher。
