# Day 37: 过程宏 (Procedural Macros)

宏（Macros）是 Rust 中一种进行**元编程**（Meta-programming）的方式，即编写“写代码的代码”。

在 Day 30 中我们提到了**声明宏**（`macro_rules!`）。今天我们将深入探讨更强大的**过程宏**（Procedural Macros）。

## 过程宏 vs 声明宏

*   **声明宏**：类似于模式匹配替换。简单，易读，但能力有限。
*   **过程宏**：函数形式，接受 Rust 代码作为输入（TokenStream），对其进行操作，并生成新的 Rust 代码作为输出。它就像是一个运行在编译期间的插件。

过程宏必须定义在独立的 crate 中，且该 crate 的类型必须标记为 `proc-macro`。

## 过程宏的三种类型

1.  **自定义派生宏 (Custom Derive Macro)**: `#[derive(CustomDerive)]`，用于结构体和枚举。
2.  **属性宏 (Attribute Macro)**: `#[CustomAttribute]`，可用于函数、结构体、模块等。
3.  **函数宏 (Function-like Macro)**: `custom!(...)`，看起来像声明宏，但更灵活。

## 本日实战：实现一个 `HelloMacro` 派生宏

我们将实现一个类似于 `Debug` 的 derive 宏。任何添加了 `#[derive(HelloMacro)]` 的结构体，都会自动获得一个 `hello_macro` 关联函数，打印出该结构体的名称。

### 核心库

编写过程宏通常需要两个核心库：

1.  **`syn`**: 将 Rust 代码（TokenStream）解析为语法树（AST），方便我们分析代码结构（如获取结构体名称）。
2.  **`quote`**: 将语法树或我们构建的数据结构转换回 Rust 代码（TokenStream）。

### 代码结构

由于过程宏必须在独立的 crate 中，本章节的项目结构如下：

*   根目录 (消费者): 定义 Trait，使用宏。
*   `hello_macro_derive/` (宏定义): 这是一个库 crate，设置了 `proc-macro = true`。

### 实现细节

在 `hello_macro_derive/src/lib.rs` 中：

```rust
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 1. 解析输入代码
    let ast = syn::parse(input).unwrap();

    // 2. 构建实现
    impl_hello_macro(&ast)
}
```

我们利用 `quote!` 宏来生成代码模板：

```rust
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

### 使用宏

```rust
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro(); // 输出: Hello, Macro! My name is Pancakes!
}
```
