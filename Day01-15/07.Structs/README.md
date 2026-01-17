# Day 07: 结构体 (Structs)

## 📝 学习目标
- 掌握如何定义和实例化结构体 (`Struct`)
- 理解元组结构体 (`Tuple Struct`) 和类单元结构体 (`Unit-Like Struct`)
- 掌握结构体方法的定义 (`impl`)
- 理解关联函数 (`Associated Functions`)

## 🎯 为什么要学这个
结构体是 Rust 中创建自定义数据类型的主要方式。
- **数据组织**：将相关联的数据组合在一起，赋予名字和意义（如 `User`, `Rectangle`）。
- **面向对象**：配合 `impl` 块，结构体提供了类似面向对象语言中"类"的功能（数据+行为），但更轻量。
- **类型安全**：通过定义不同的结构体，编译器可以帮你区分看似相同但含义不同的数据（如 `Miles(u32)` 和 `Kilometers(u32)`）。

## 📖 核心概念

### 1. 经典结构体 (Classic Structs)
最常用的形式，每个字段都有名字。

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

**实例化**：
```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

**简写语法**：当变量名与字段名相同时。
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email, // 等同于 email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

**更新语法**：基于旧实例创建新实例。
```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1 // 必须放在最后
};
```
⚠️ **注意**：如果使用了更新语法，且移动了堆上数据（如 `String`），旧实例可能无法再被完整使用。

### 2. 元组结构体 (Tuple Structs)
只有字段类型，没有字段名。适用于只需要区分类型但不需要命名字段的场景。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
访问字段使用索引：`black.0`。

### 3. 类单元结构体 (Unit-Like Structs)
没有字段。常用于实现 Trait，但不需要存储数据。
```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```

### 4. 方法 (Methods)
定义在 `impl` 块中，第一个参数是 `self`。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self 是 self: &Self 的简写，表示借用实例
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### 5. 关联函数 (Associated Functions)
定义在 `impl` 块中，但不以 `self` 作为参数。通常用作构造函数。

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
// 调用方式
let sq = Rectangle::square(3);
```

## 💻 代码示例

### 示例: 完整的矩形程序
```rust
#[derive(Debug)] // 允许使用 {:?} 打印
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };

    println!("rect1 is {:?}", rect1);
    println!("The area of rect1 is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
```

## 🏋️ 练习题

我们为你准备了关于结构体的定义、实例化和方法实现的练习。

- **练习 1**: 定义书库
- **练习 2**: 结构体更新语法
- **练习 3**: 矩形计算方法

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: `&self`, `&mut self`, `self` 有什么区别？
- `&self`: **不可变借用**。最常用，只读取数据。
- `&mut self`: **可变借用**。需要修改结构体内部数据时使用。
- `self`: **夺取所有权**。通常用于将结构体转换为另一种类型，转换后原变量不可用。

### Q2: 为什么不能直接打印结构体 `println!("{}", user)`？
A: `println!` 默认使用 `Display` trait（`{}`），结构体默认没有实现它。可以使用 `#[derive(Debug)]` 自动实现 `Debug` trait，然后用 `{:?}` 或 `{:#?}` 打印。

## 💡 最佳实践
- **大写驼峰命名**：结构体名称遵循 `PascalCase`（如 `UserAccount`）。
- **字段蛇形命名**：字段名称遵循 `snake_case`（如 `user_id`）。
- **优先使用 String**：在结构体中存储文本时，优先使用 `String` 拥有所有权，避免复杂的生命周期问题（直到你确实需要引用）。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 使用结构体组织数据](https://doc.rust-lang.org/book/ch05-00-structs.html)

## 📚 本节要点回顾
- `struct` 关键字定义结构体。
- `impl` 块定义方法和关联函数。
- `.` 运算符访问字段和调用方法。
- `#[derive(Debug)]` 用于调试打印。

## ⏭️ 下一步
有了结构体，我们还可以通过另一种强大的方式来定义数据：枚举。

下一节: [Day 08: 枚举与模式匹配](../08.Enums/README.md)
