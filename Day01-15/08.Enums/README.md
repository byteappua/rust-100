# Day 08: 枚举 (Enums) 与模式匹配

枚举（Enumerations），通常称为 Enums，允许你通过列举可能的成员（Variants）来定义一个类型。

## 1. 定义枚举

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## 2. Option 枚举

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## 3. match 控制流运算符

```rust
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}
```

## 4. if let

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```
