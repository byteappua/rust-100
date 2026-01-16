# Day 08: 枚举 (Enums) 与模式匹配

枚举（Enumerations），通常称为 Enums，允许你通过列举可能的成员（Variants）来定义一个类型。

## 1. 定义枚举

### 基本枚举

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### 带有数据的枚举

枚举的成员可以包含不同类型和数量的数据。

```rust
enum IpAddr {
    V4(u8, u8, u8, u8), // 像元组结构体
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

甚至可以包含结构体：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### 枚举的方法

也可以像结构体一样，使用 `impl` 为枚举定义方法。

```rust
impl Message {
    fn call(&self) {
        // ...
    }
}
```

## 2. Option 枚举

Rust 没有 Null 值，但是提供了 `Option<T>` 枚举来表示一个值可能存在，也可能不存在。它定义在标准库中。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

使用 `Option`：

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None; // 必须指定类型，因为无法从 None 推断 T
```

要获取 `T` 的值，通常需要使用模式匹配。

## 3. match 控制流运算符

`match` 允许我们将一个值与一系列的模式进行比较，并根据匹配的模式执行相应的代码。

### 基本用法

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 绑定值的模式

如果枚举成员包含数据，`match` 可以提取它。

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### 匹配 Option<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### 穷尽性 (Exhaustiveness)

`match` 的分支必须覆盖所有可能的情况。如果漏掉一个分支，编译器会报错。

### 通配符 `_` 与 `other`

如果我们只关心部分情况，可以使用 `_`（下划线）匹配所有其他情况并忽略值，或者使用变量名捕获值。

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // 捕获值
    // _ => reroll(), // 或者使用 _ 忽略值
}
```

## 4. if let 控制流

`if let` 语法让我们以一种不那么冗长的方式处理只匹配一个模式的值而忽略其他模式的情况。

```rust
let config_max = Some(3u8);

// 使用 match
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

// 使用 if let
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
} else {
    println!("Config max is None");
}
```

`if let` 可以看作是 `match` 的语法糖，当只需要处理一种模式（或配合 `else` 处理不匹配的情况）时使用。
