# Day 30: 高级函数与宏 (Advanced Functions & Macros)

## 1. 函数指针

普通函数可以直接传递给期望函数参数的代码。类型是 `fn`（小写），这被称为**函数指针**。

```rust
fn add_one(x: i32) -> i32 { x + 1 }

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```

区别：
*   `Fn`, `FnMut`, `FnOnce`: 是 Trait，闭包实现了这些 Trait。
*   `fn`: 是具体类型，没有运行时环境（不能捕获环境），只能指向代码。
*   `fn` 类型实现了 `Fn`, `FnMut`, `FnOnce`，所以可以在需要闭包的地方传函数指针。

## 2. 返回闭包

闭包是用 Trait 表示的，大小不确定，所以不能直接作为返回值返回 `dyn Fn`。必须放在指针后面，通常是 `Box`。

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## 3. 宏 (Macros)

Rust 的宏非常强大，属于**元编程**。

### 声明宏 (Declarative Macros) `macro_rules!`

类似于 `match` 表达式。

```rust
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

*   `$x:expr`: 匹配任何表达式并绑定到 `$x`。
*   `$(...),*`: 匹配括号内的模式零次或多次，使用 `,` 分隔。

### 过程宏 (Procedural Macros)

更像函数，接受代码作为输入，生成新代码作为输出。
*   派生宏 (`#[derive]`)
*   属性宏
*   函数宏
