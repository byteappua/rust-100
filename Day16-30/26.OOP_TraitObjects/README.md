# Day 26: 面向对象特性 - Trait 对象

Rust 不是传统的面向对象语言（没有继承），但可以使用 **Trait 对象** 来实现多态性。

## 1. Trait 对象 (Trait Objects)

如果我们需要一个集合，其中包含不同类型但都实现了某个 Trait 的对象，我们可以使用 Trait 对象。

语法：`Box<dyn Trait>` 或 `&dyn Trait`。

`dyn` 关键字强调这是一个动态调度的类型。

## 2. 动态调度 (Dynamic Dispatch)

当我们在泛型中使用 Trait Bound 时（如 `fn foo<T: Trait>(t: T)`），编译器会进行**单态化 (Monomorphization)**，也就是**静态调度**。编译器为每个具体类型生成代码，性能好，但灵活性受限（集合中只能是一种类型）。

使用 Trait 对象时，Rust 在运行时查找 Trait 方法的实现（通过 vtable），这就是**动态调度**。这会带来微小的运行时开销，但允许更大的灵活性。

## 3. 对象安全 (Object Safety)

只有**对象安全**的 Trait 才能组成 Trait 对象。如果一个 Trait 的所有方法都满足以下条件，则它是对象安全的：
1.  返回值类型不是 `Self`。
2.  方法没有任何泛型类型参数。

```rust
trait Draw {
    fn draw(&self);
}

struct Screen {
    // 这里可以存放任何实现了 Draw 的类型
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```
