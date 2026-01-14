# Day 29: 高级类型 (Advanced Types)

## 1. Newtype 模式

使用元组结构体创建一个新类型。即使内部类型一样，新类型也被视为完全不同的类型。这提供了静态类型检查的安全性。

```rust
struct Millimeters(u32);
struct Meters(u32);
```

## 2. 类型别名 (Type Aliases)

使用 `type` 关键字给现有类型起别名。别名**不是**新类型，它只是原类型的另一个名字。

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y); // 合法，因为 Kilometers 就是 i32
```

主要用途是减少长类型名的重复：

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;
```

## 3. Never Type (!)

Rust 有一个特殊的类型 `!`，称为 **Never Type**，表示永远不会有值返回。

*   `panic!` 返回 `!`。
*   `continue` 和 `break` 表达式的值是 `!`。
*   `loop` 如果没有 break，也返回 `!`。

`!` 可以被强制转换为任何类型，这使得它在 `match` 分支中非常有用。

```rust
let num: u32 = match guess.parse() {
    Ok(num) => num,
    Err(_) => continue, // continue 返回 !，可以匹配 u32
};
```

## 4. 动态大小类型 (DSTs) 与 Sized Trait

Rust 中大多数类型都是 **Sized**（编译时已知大小）。但也存在 **DSTs** (Dynamically Sized Types)，如 `str` 和 `[T]`。

我们不能直接使用 DST，只能通过引用或指针使用它们（`&str`, `Box<str>`）。引用本身是有固定大小的（指针+长度）。

### Sized Trait

只有已知大小的类型才实现了 `Sized` Trait。
泛型函数默认有一个隐式的限定 `T: Sized`。

如果要接受 DST，需要使用特殊语法 `T: ?Sized`（可能是 Sized，也可能不是）。

```rust
fn generic<T: ?Sized>(t: &T) { ... }
```
