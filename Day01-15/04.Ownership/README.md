# Day 04: 所有权 (Ownership)

所有权 (Ownership) 是 Rust 最独特的特性，也是它在没有垃圾回收 (GC) 的情况下保证内存安全的关键所在。如果不理解所有权，就无法编写 Rust 代码。

## 1. 内存管理方式对比

所有程序都必须管理其运行时使用计算机内存的方式。

*   **垃圾回收 (GC)**: 语言在运行时定期查找不再使用的内存并释放（如 Java, Python, Go）。优点是省心，缺点是可能有性能损耗。
*   **手动管理**: 程序员必须显式分配和释放内存（如 C, C++）。优点是控制力强，缺点是容易出错（内存泄漏、双重释放、野指针）。
*   **所有权系统**: Rust 采用的方式。内存通过一个所有权系统来管理，其中包含一组在**编译时**检查的规则。运行时没有额外开销。

## 2. 栈 (Stack) 与 堆 (Heap)

理解所有权需要先了解栈和堆。

*   **栈 (Stack)**:
    *   **后进先出 (LIFO)**。
    *   存储的数据必须拥有**已知且固定的大小**。
    *   分配速度极快（只需移动栈指针）。
    *   例如：`i32`, `bool`, `char`。
*   **堆 (Heap)**:
    *   存储编译时大小未知或可能变化的数据。
    *   分配需要向操作系统请求空间，返回一个指针。
    *   访问速度比栈慢（需要通过指针跳转）。
    *   例如：`String`, `Vec`。

## 3. 所有权规则 (The Rules)

1.  Rust 中的每一个值都有一个被称为其 **所有者 (Owner)** 的变量。
2.  值在任一时刻**有且只有一个**所有者。
3.  当所有者（变量）离开作用域，这个值将被丢弃 (Dropped)。

## 4. 变量作用域

```rust
{                      // s 在这里无效
    let s = "hello";   // s 开始有效
    // 使用 s
}                      // 作用域结束，s 不再有效
```

## 5. 移动 (Move)

当我们将一个变量赋值给另一个变量时，Rust 的行为取决于数据类型。

### 栈上的数据：拷贝 (Copy)

对于整型等固定大小类型，赋值是完全的拷贝。

```rust
let x = 5;
let y = x;
// x 和 y 都是有效的，且等于 5
```

### 堆上的数据：移动 (Move)

对于 `String` 这样的复杂类型：

```rust
let s1 = String::from("hello");
let s2 = s1;
```

内存表现：
1.  `s1` 在栈上存储了指向堆内存的指针、长度和容量。
2.  `s2 = s1` 拷贝了栈上的指针、长度和容量，但**没有**复制堆上的数据。
3.  **关键点**：Rust 认为 `s1` 不再有效！

```rust
// println!("{}, world!", s1); // 编译错误！s1 已经被移动
```

这解决了**二次释放 (Double Free)** 问题：当 `s2` 和 `s1` 离开作用域时，只有 `s2` 会尝试释放内存，因为 `s1` 已经被标记为无效。

## 6. 克隆 (Clone)

如果我们确实需要深度复制堆上的数据，可以使用 `clone` 方法。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
// 堆上的数据被复制了一份
println!("s1 = {}, s2 = {}", s1, s2); // 正常运行
```

## 7. Copy Trait

如果一个类型实现了 `Copy` trait，一个变量赋值给另一个变量后，原来的变量仍然有效。
任何需要分配内存或某种形式资源的类型都不会实现 `Copy`。

常见的 `Copy` 类型：
*   所有整数类型：`u32`, `i32` 等。
*   布尔类型：`bool`。
*   所有浮点数类型：`f64`, `f32`。
*   字符类型：`char`。
*   元组：当且仅当其包含的类型也都是 `Copy` 的时候。例如 `(i32, i32)` 是，但 `(i32, String)` 不是。

## 8. 所有权与函数

将值传递给函数与给变量赋值的语义完全一样：要么移动，要么复制。

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里
                                    // ... 所以这里不再有效

    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里 x 先移出作用域，然后是 s。但 s 的值已被移走，所以不会有什么特别的操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里 some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里 some_integer 移出作用域。不会有特殊操作
```

## 9. 返回值与作用域

返回值也可以转移所有权。

```rust
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回 a_string 并移出给调用的函数
}
```
