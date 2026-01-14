# Day 12: 泛型 (Generics)

泛型是具体类型或其他属性的抽象替代。在编译时，Rust 会将泛型代码“单态化”（Monomorphization），这意味着泛型不会带来运行时的性能损耗。

## 1. 函数定义中的泛型

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

这里 `T` 是类型参数的名称。`PartialOrd` 是一个 trait bound（特征约束），表示 `T` 必须是可以比较大小的类型。

## 2. 结构体定义中的泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

如果要支持不同类型：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

## 3. 枚举定义中的泛型

我们之前见过的 `Option` 和 `Result` 就是泛型枚举。

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 4. 方法定义中的泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

可以为特定类型实现方法：

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
