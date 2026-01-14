# Day 07: 结构体 (Structs)

结构体（Struct）让你能够将多个相关的值组合成一个有意义的命名组合。

## 1. 定义与实例化结构体

### 定义结构体

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

### 实例化结构体

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

## 2. 元组结构体 (Tuple Structs)

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

## 3. 类单元结构体 (Unit-Like Structs)

```rust
struct AlwaysEqual;
```

## 4. 方法语法

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
