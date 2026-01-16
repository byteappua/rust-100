# Day 48: 命令处理架构

将底层的 `Frame` 解析出来后，我们需要将其转换为具体的**业务命令**。这是一种典型的**命令模式 (Command Pattern)**。

## 1. 为什么需要这一层？

`Frame` 只是协议层面的数据表示（数组、字符串、整数）。业务逻辑不应该直接操作 `Frame`，而应该操作语义明确的 `Command`。

例如，`SET key value` 命令在 RESP 中是一个由 3 个 Bulk Strings 组成的 Array。
我们需要验证：
1.  确实是 Array 类型。
2.  第一个元素是 "SET"。
3.  总共有 3 个元素。
4.  提取出 `key` 和 `value`。

## 2. 定义 Command 枚举

```rust
pub enum Command {
    Get(Get),
    Set(Set),
    Publish(Publish),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
    Unknown(Unknown),
}
```

每个具体的命令都是一个独立的结构体，例如 `Set`：

```rust
pub struct Set {
    pub key: String,
    pub value: Bytes,
}
```

## 3. 实现 from_frame

我们为 `Command` 实现一个方法，将 `Frame` 转换为 `Command`：

```rust
impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        // ...
    }
}
```

这样，我们的主流程就变成了：
`Bytes -> Frame -> Command -> Execute -> Frame -> Bytes`
