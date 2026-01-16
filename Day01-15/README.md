# Rust 基础 (Day 01 - 15)

这个目录涵盖了 Rust 编程语言的基础知识，大致对应《Rust 程序设计语言》的前 10-11 章。通过这 15 天的学习，你将掌握 Rust 的核心语法、所有权系统、类型系统以及基本的项目组织方式。

## 目录

*   **[Day 01: Introduction (初识 Rust)](01.Introduction/README.md)**
    *   环境搭建、安装 Rust
    *   Hello World
    *   Cargo 包管理器简介
*   **[Day 02: Variables and Types (变量与数据类型)](02.VariablesAndTypes/README.md)**
    *   变量与可变性
    *   标量类型 (整型, 浮点型, 布尔型, 字符)
    *   复合类型 (元组, 数组)
*   **[Day 03: Functions and Control Flow (函数与控制流)](03.FunctionsAndControlFlow/README.md)**
    *   函数定义、参数与返回值
    *   if 表达式
    *   循环 (loop, while, for)
*   **[Day 04: Ownership (所有权)](04.Ownership/README.md)**
    *   所有权规则
    *   堆与栈
    *   移动 (Move)、克隆 (Clone) 与 拷贝 (Copy)
*   **[Day 05: References and Borrowing (引用与借用)](05.ReferencesAndBorrowing/README.md)**
    *   不可变引用与可变引用
    *   引用规则
    *   悬垂引用
*   **[Day 06: Slices (切片)](06.Slices/README.md)**
    *   字符串切片
    *   其他类型的切片
*   **[Day 07: Structs (结构体)](07.Structs/README.md)**
    *   定义与实例化结构体
    *   方法与关联函数
*   **[Day 08: Enums (枚举)](08.Enums/README.md)**
    *   定义枚举
    *   `Option` 枚举
    *   `match` 控制流运算符
    *   `if let` 语法
*   **[Day 09: Modules (模块系统)](09.Modules/README.md)**
    *   包与 Crates
    *   定义模块
    *   路径与 `use` 关键字
    *   可见性 (`pub`)
*   **[Day 10: Common Collections (常用集合)](10.CommonCollections/README.md)**
    *   Vector (`Vec<T>`)
    *   String (`String`)
    *   HashMap (`HashMap<K, V>`)
*   **[Day 11: Error Handling (错误处理)](11.ErrorHandling/README.md)**
    *   `panic!` 与不可恢复错误
    *   `Result` 与可恢复错误
    *   错误传播与 `?` 运算符
*   **[Day 12: Generics (泛型)](12.Generics/README.md)**
    *   函数、结构体、枚举中的泛型
    *   单态化
*   **[Day 13: Traits (特征)](13.Traits/README.md)**
    *   定义与实现 Trait
    *   Trait Bounds
    *   `impl Trait`
*   **[Day 14: Lifetimes (生命周期)](14.Lifetimes/README.md)**
    *   生命周期注解语法
    *   生命周期省略规则
    *   静态生命周期
*   **[Day 15: Testing (自动化测试)](15.Testing/README.md)**
    *   编写单元测试
    *   `assert!` 宏
    *   运行测试
    *   集成测试

## 学习建议

建议按照顺序学习，每一天的内容都建立在前一天的基础上。对于每个主题，不仅要阅读文档，还要亲手编写并运行代码。
