# Day 25: 并发编程 - Send 与 Sync Trait

## 📝 学习目标

- 理解 **Marker Traits** (标记 Trait) 的概念
- 掌握 **`Send`** (所有权传送) 与 **`Sync`** (线程间共享) 的区别
- 理解 "T is Sync if and only if &T is Send" 的逻辑关系
- 能够识别哪些类型是线程安全的，哪些不是

## 🎯 核心概念：安全的基石

Rust 的并发安全不仅仅靠可以 "锁" (Mutex) 或者 "通道" (Channel)，更底层的是靠 **类型系统**。
`Send` 和 `Sync` 是 Rust 编译器在多线程环境下保驾护航的两个 "门神"。它们告诉编译器：这个类型能不能安全地送到另一个线程？能不能被多个线程同时看？

### 关系逻辑图

```mermaid
graph TD
    Type[类型 T] --> CheckSend{能在线程间移动吗?}
    CheckSend --Yes--> Send[impl Send for T]
    CheckSend --No--> NotSend[非 Send (如 Rc, 裸指针)]
    
    Type --> CheckSync{能被多线程同时读取吗?}
    CheckSync --Yes--> Sync[impl Sync for T]
    CheckSync --No--> NotSync[非 Sync (如 RefCell, Rc)]
    
    Sync -.->|蕴含| SendRef[impl Send for &T]
    
    style Send fill:#ccffcc
    style Sync fill:#e1f5fe
    style NotSend fill:#ffcccc
    style NotSync fill:#ffcccc
```

---

## 1. Send: 所有权的转移

**`Send` 标记表明类型 T 的所有权可以在线程间转移。**

- **绝大多数类型**都是 `Send` 的（如 `i32`, `String`, `Box<T>`, `Mutex<T>`）。
- **例外**：`Rc<T>`。因为如果把 `Rc` 移到另一个线程，两个线程同时修改引用计数会导致数据竞争（非原子操作）。

```rust
use std::thread;
use std::rc::Rc;

let r = Rc::new(5);
// ❌ 编译错误！Rc<i32> 没有实现 Send，不能 move 进线程
thread::spawn(move || {
    println!("{}", r);
});
```

---

## 2. Sync: 线程间共享

**`Sync` 标记表明类型 T 可以被多个线程安全地共享引用 (通过 `&T`)。**

### 黄金法则
>
> **T is `Sync` if and only if `&T` is `Send`.**
> (如果 T 的引用可以被送往另一个线程，那么 T 就是 Sync 的)

这听起来有点绕，但逻辑很简单：如果我在线程 A 有一个 `&T`，我把它传给线程 B，那现在 A 和 B 都有 `&T`，也就是都在访问同一个 T。这就叫 "共享"。

- **`Mutex<T>`** 是 `Sync` 的（它可以被多线程共享）。
- **`RefCell<T>`** 不是 `Sync` 的（它的运行时借用检查不是线程安全的，多线程同时 `borrow_mut` 会导致数据竞争）。

---

## 📊 线程安全矩阵 (Thread Safety Matrix)

| 类型 | Send? | Sync? | 说明 |
| :--- | :--- | :--- | :--- |
| **`i32`, `bool`** | ✅ | ✅ | 简单值拷贝，哪里都能去 |
| **`String`, `Vec<T>`** | ✅ | ✅ | 独占所有权，Move 没问题 |
| **`Rc<T>`** | ❌ | ❌ | 引用计数非原子，彻底不安全 |
| **`Arc<T>`** | ✅ | ✅ | 原子引用计数，专门为多线程设计 |
| **`RefCell<T>`** | ✅ | ❌ | 可以 Move，但不能在线程间共享引用 (因为内部检查不安全) |
| **`Mutex<T>`** | ✅ | ✅ | 内部有锁，可以安全共享 |
| **`*const T`** (裸指针) | ❌ | ❌ | 编译器无法追踪裸指针，默认不安全 |

---

## 3. 手动实现 (Unsafe)

通常你不需要手动实现这些 Trait，编译器会自动为由 Send/Sync 类型组成的结构体自动推导。
但如果你编写底层代码（如使用裸指针实现自己的 Vec），可能需要手动通过 `unsafe` 告诉编译器："相信我，这是安全的"。

```rust
struct MyBox(*mut u8);

// 只有当我确信你是由于某种原因安全的时候...
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}
```

---

## 🏋️ 练习题

👉 **[点击这里查看练习题](./exercises/README.md)**

1. **Rc 挑战**: 尝试在 `thread::spawn` 中使用 `Rc`, 阅读编译器给出的错误信息。
2. **验证 RefCell**: 尝试将 `&RefCell<i32>` 传递给另一个线程，观察报错。
3. **Wrapper**: 创建一个包含 `Rc` 的 NewType 结构体，手动为其实现 `Send` (虽然这很不安全)，骗过编译器并强行运行，观察可能发生的崩溃或数据错误（仅用于教学！）。

---

## 💡 最佳实践

1. **相信编译器**: 如果编译器报 "T cannot be sent between threads safely"，通常是你错误地使用了 `Rc` 或 `RefCell`，或者忘记用 `Arc`/`Mutex` 包装了。
2. **Arc<Mutex<T>>**: 记住这个公式。`Arc` 提供 Send/Sync (让多线程持有)，`Mutex` 提供内部可变性 (让多线程修改)。

---

## ⏭️ 下一步

关于并发的内存安全我们已经讲透了。
现在，让我们换个脑子，回到 Rust 的语言特性本身。Rust 不是面向对象的语言，但它确实有一些面向对象的特性。

下一节: [Day 26: 面向对象特性 (OO Features)](../26.OOP_TraitObjects/README.md)
