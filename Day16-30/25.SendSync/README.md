# Day 25: 并发编程 - Send 与 Sync Trait

`Send` 和 `Sync` 是 Rust 并发安全的基石。它们是 **Marker Traits**（标记 Trait），没有任何方法，只是用来标记类型具有某些并发属性。

## 1. Send Trait

`Send` 标记表明类型的所有权可以在线程间转移。

*   几乎所有的 Rust 类型都是 `Send` 的。
*   主要例外：`Rc<T>`。因为 `Rc` 的引用计数不是原子的，在多线程间转移会导致计数错误。
*   `Raw Pointers` (裸指针) 也不是 `Send` 的。

如果一个类型完全由 `Send` 的类型组成，那么它自动就是 `Send` 的。

## 2. Sync Trait

`Sync` 标记表明类型可以被多个线程安全地共享引用（即 `&T` 是 `Send` 的）。

*   基本类型是 `Sync` 的。
*   `Mutex<T>` 是 `Sync` 的。
*   `RefCell<T>` 不是 `Sync` 的，因为它的运行时借用检查不是线程安全的。
*   `Rc<T>` 不是 `Sync` 的。

## 3. 手动实现

通常不需要手动实现 `Send` 和 `Sync`，编译器会自动推导。手动实现是不安全的（`unsafe`），通常只在编写底层并发原语时需要。

```rust
// 仅作演示，通常由编译器自动推导
unsafe impl Send for MyType {}
unsafe impl Sync for MyType {}
```

## 4. 编译器检查

Rust 编译器会利用这些 Trait 来检查并发代码的安全性。例如 `thread::spawn` 的签名要求闭包捕获的变量必须是 `Send` 的。

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
```
