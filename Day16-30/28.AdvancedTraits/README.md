# Day 28: 高级 Trait (Advanced Traits)

## 1. 关联类型 (Associated Types)

关联类型是 Trait 中的类型占位符。实现 Trait 时需要指定具体的类型。

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

与泛型 `trait Iterator<T>` 的区别在于：对于一个具体的类型，只能有一个关联类型的实现。而泛型可以对同一类型实现多次（针对不同的 T）。

## 2. 默认泛型类型参数与运算符重载

Rust 允许运算符重载，这是通过实现 `std::ops` 下的 Trait 来完成的。

```rust
use std::ops::Add;

struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
```

`Add` Trait 的定义使用了默认泛型参数：`trait Add<Rhs=Self> { ... }`。如果不指定 `Rhs`，默认就是 `Self`。

## 3. 完全限定语法 (Fully Qualified Syntax)

当同一类型实现了两个具有相同方法名的 Trait 时，需要消除歧义。

```rust
trait Pilot { fn fly(&self); }
trait Wizard { fn fly(&self); }

struct Human;
impl Pilot for Human { ... }
impl Wizard for Human { ... }

let person = Human;
Pilot::fly(&person);
Wizard::fly(&person);
```

如果是关联函数（没有 `self`）：`<Type as Trait>::function(args)`。

## 4. Supertraits (父 Trait)

有时一个 Trait 依赖于另一个 Trait。

```rust
trait OutlinePrint: fmt::Display { ... }
```

实现 `OutlinePrint` 的类型必须也实现 `Display`。

## 5. Newtype 模式

孤儿规则（Orphan Rule）：只有当 Trait 或类型定义在本地 crate 时，才能为类型实现 Trait。
绕过方法：使用 **Newtype 模式**（元组结构体包装）。

```rust
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper { ... }
```
