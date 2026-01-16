# Day 07: 结构体 (Structs)

结构体（Struct）是一种自定义数据类型，让你能够将多个相关的值组合成一个有意义的命名组合。这就好比面向对象语言中的对象的数据属性。

## 1. 定义与实例化结构体

### 定义结构体

使用 `struct` 关键字。

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

### 实例化结构体

需要为每个字段赋值。顺序不重要。

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

### 获取结构体字段的值

使用点号 `.`。如果结构体实例是可变的，我们也可以修改字段的值。

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

### 字段初始化简写语法

当变量名与字段名相同时，可以简化写法。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,    // 等同于 email: email
        username, // 等同于 username: username
        active: true,
        sign_in_count: 1,
    }
}
```

### 结构体更新语法

可以使用旧实例的大部分值来创建一个新实例。

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

注意：`..user1` 必须放在最后。这会发生所有权的转移（对于实现了 Copy 的字段则是复制）。在上面的例子中，如果 `user1` 的 `username` 字段（String 类型）被移动到了 `user2`，那么 `user1` 在这之后就部分无效了。

## 2. 元组结构体 (Tuple Structs)

元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

访问元组结构体字段使用索引：`black.0`。

## 3. 类单元结构体 (Unit-Like Structs)

没有任何字段的结构体。通常用于在某个类型上实现 Trait，但不需要存储数据。

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

## 4. 结构体数据的所有权

在上面的 `User` 结构体定义中，我们使用了 `String` 类型而不是 `&str`。这是一个有意为之的选择，因为我们希望结构体拥有它所有的数据。
如果要存储引用，需要使用**生命周期 (Lifetimes)**，这在后续章节会讨论。

## 5. 方法语法 (Method Syntax)

方法与函数类似，但它们是在结构体的上下文中定义的（或者是枚举、Trait 对象）。方法的第一个参数总是 `self`。

### 定义方法

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self 是 self: &Self 的简写
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

### 方法参数 `self`

*   `&self`: 借用结构体实例，不可变（最常用）。
*   `&mut self`: 借用结构体实例，可变（用于修改实例）。
*   `self`: 获取结构体实例的所有权（通常用于转换类型，转换后原实例不可用）。

### 关联函数 (Associated Functions)

`impl` 块中定义的函数不一定非要以 `self` 作为参数。这种函数被称为关联函数（因为它们与结构体相关联）。最常见的例子是 `String::from`。
通常用于构造函数。

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

调用关联函数使用 `::` 语法：

```rust
let sq = Rectangle::square(3);
```

## 6. 打印结构体

默认情况下，结构体没有实现 `Display` trait，所以不能直接用 `{}` 打印。
但是可以通过 `#[derive(Debug)]` 为结构体自动派生 `Debug` trait。

```rust
let rect1 = Rectangle {
    width: 30,
    height: 50,
};

println!("rect1 is {:?}", rect1); // 输出单行调试信息
println!("rect1 is {:#?}", rect1); // 输出格式化（多行）调试信息
```
