# Day 26: 面向对象特性 - Trait 对象

## 📝 学习目标

- 理解 Rust 中的 **多态 (Polymorphism)** 实现方式
- 掌握 **Trait Object** (`dyn Trait`) 与 **动态调度**
- 理解 **胖指针 (Fat Pointer)** 与 **vtable** 的底层原理
- 掌握 **对象安全 (Object Safety)** 的规则

## 🎯 核心概念：Rust 的“继承”

Rust 不是传统的 OOP 语言（没有 `class` 和继承），但通过 **Trait Objects**，我们可以实现运行时多态，即：
**“我不在乎你是谁，我只在乎你能做什么（Trait）。”**

### 1. 静态调度 vs 动态调度

Rust 提供了两种多态方式：

| 特性 | 泛型 (Generics) | Trait 对象 (Trait Objects) |
| :--- | :--- | :--- |
| **语法** | `fn foo<T: Draw>(x: T)` | `fn foo(x: &dyn Draw)` |
| **术语** | **静态调度** (Static Dispatch) | **动态调度** (Dynamic Dispatch) |
| **原理** | **单态化** (编译时生成多份代码) | **Vtable** (运行时查表) |
| **优点** | 性能极致 (内联优化) | 灵活性强 (异构集合) |
| **缺点** | 二进制体积变大，编译慢 | 运行时微小开销，无法内联 |
| **适用** | 绝大多数场景 | 需要存储不同类型的集合时 |

---

## 2. 深入底层：胖指针 (Fat Pointer)

当你使用 `&dyn Trait` 或 `Box<dyn Trait>` 时，Rust 实际上在幕后操作一个 **胖指针**。
普通的指针只有 64 位（存储地址），但胖指针有 128 位：

```mermaid
graph LR
    subgraph Stack [栈 Stack]
        OBJ[Box dyn Draw]
        PTR1[Data Ptr]
        PTR2[Vtable Ptr]
        OBJ --> PTR1
        OBJ --> PTR2
    end
    
    subgraph Heap [堆 Heap]
        DATA[具体数据 (Button)]
    end
    
    subgraph StaticMemory [静态内存]
        VTABLE[Vtable (Button 实现的 Draw 方法表)]
        FUNC[draw() 函数代码]
    end
    
    PTR1 --> DATA
    PTR2 --> VTABLE
    VTABLE --> FUNC
    
    style Stack fill:#ccffcc
    style Heap fill:#e1f5fe
    style StaticMemory fill:#fff9c4
```

- **Data Ptr**: 指向实际的数据（如 `Button` 结构体）。
- **Vtable Ptr**: 指向虚函数表 (Virtual Method Table)，里面记录了 `Button` 为 `Draw` Trait 实现的所有方法的地址。

这就是为什么这叫 **动态调度**：程序运行时，需要先通过 vtable 找到函数地址，然后再跳转执行。

---

## 💻 代码实战：GUI 库

假设我们要写一个 GUI 库，界面上有一堆组件（按钮、文本框），它们都实现了 `Draw` trait。

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {}", self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box with options: {:?}", self.options);
    }
}

struct Screen {
    // 关键点：这里必须用 Box<dyn Draw>，因为 Button 和 SelectBox 大小不同
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // 动态调度：根据具体类型调用对应的 draw
        }
    }
}
```

---

## 3. 对象安全 (Object Safety)

不是所有的 Trait 都能变成 Trait 对象。只有 **对象安全** 的 Trait 才可以。
如果你尝试把不安全的 Trait 变成 `dyn Trait`，编译器会报错。

**主要规则（必须同时满足）**：

1. **方法的返回值类型不能是 `Self`**。
    - 原因：运行时 `Self` 已经被擦除了，编译器不知道 `Self` 到底多大。
2. **方法不能有泛型参数**。
    - 原因：泛型方法需要单态化，需要编译时生成代码，而 Trait Object 是运行时决定的。

**错误示例**: `Clone` Trait 就**不是**对象安全的，因为它有 `fn clone(&self) -> Self`。
所以你不能写 `Box<dyn Clone>`。

---

## 🏋️ 练习题

👉 **[点击这里查看练习题](./exercises/README.md)**

1. **GUI 扩展**: 实现一个新的组件 `TextField`，并将其加入到 `Screen` 中运行。
2. **对象安全**: 尝试定义一个带有泛型方法的 Trait，然后尝试将其用作 Trait Object，观察编译器的报错信息。
3. **State 模式**: 使用 Trait Object 实现经典的 State 设计模式（如博客文章的草稿 -> 审核 -> 发布状态流转）。

---

## 💡 最佳实践

1. **优先泛型**: 默认情况下使用泛型 (`fn foo<T: Component>`)。只有当你确实需要在一个容器里存不同类型的数据时，才使用 Trait Object。
2. **Box vs &**: `Box<dyn Trait>` 拥有所有权（堆上），`&dyn Trait` 只是借用。
3. **Sized**: Trait Object 是 `!Sized` (大小不确定) 的，所以必须总是在指针后面使用它 (`&`, `Box`, `Rc` 等)。

---

## ⏭️ 下一步

到目前为止，我们学习的模式匹配都比较基础 (`match` 枚举)。Rust 的模式匹配功能远不止于此。
下一节，我们将深入探索模式匹配的高级用法，如解构结构体、忽略值、匹配守卫等。

下一节: [Day 27: 模式匹配 (Patterns and Matching)](../27.Patterns/README.md)
