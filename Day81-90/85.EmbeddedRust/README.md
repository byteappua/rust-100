# Day 85: 嵌入式 Rust 简介 (Embedded Rust Introduction)

## 简介
嵌入式 Rust 开发允许我们在没有操作系统（`no_std`）的环境下直接控制硬件。Rust 的所有权模型和类型系统在嵌入式领域提供了极高的安全性和便利性。

## 核心概念

1.  **no_std**: 嵌入式环境通常没有完整的标准库支持（如堆分配、文件系统、线程），因此需要使用 `#![no_std]` 属性禁用标准库，仅使用 `core` 库。
2.  **embedded-hal**: 这是一个硬件抽象层（Hardware Abstraction Layer）的 Trait 集合。驱动程序编写者只需针对 `embedded-hal` 编写代码，即可在任何实现了该 HAL 的芯片上运行。
3.  **PAC (Peripheral Access Crate)**: 提供对芯片寄存器的底层访问。
4.  **Board Support Crate (BSP)**: 针对特定开发板的封装。

## 本日项目: `embedded-demo`

由于在通用计算机（Host）上无法直接运行针对特定微控制器（如 STM32, ESP32）的代码，本项目展示了如何编写**平台无关**的驱动程序，并在 Host 上进行模拟运行。

### 项目结构

-   `src/lib.rs`: 包含一个通用的 `Blinky` 驱动。它依赖于 `embedded-hal`，不依赖于具体硬件，也不依赖 `std`。这意味着这段代码可以直接编译并运行在真正的微控制器上。
-   `src/main.rs`: 这是一个在 Host (Windows/Linux/macOS) 上运行的程序。它实现了 `embedded-hal` 定义的 Traits (`OutputPin`, `DelayNs`) 的模拟版本（Mock），并使用 `Blinky` 驱动来控制这个“虚拟硬件”。

### 运行方式

在 `embedded-demo` 目录下运行：

```bash
cargo run
```

你将看到终端输出模拟的 LED 开关状态，这证明了我们的驱动逻辑是正确的。

### 如何迁移到真实硬件？

要在真实硬件（例如 STM32F103）上运行 `Blinky`，你需要：

1.  创建一个新的 Binary Target (通常在 `examples/` 下)。
2.  引入芯片的 HAL 库（如 `stm32f1xx-hal`）。
3.  在 `main` 函数中初始化硬件，获取真正的 GPIO 引脚和延时器。
4.  将这些真正的硬件对象传递给 `Blinky::new()`。
5.  使用 `cargo build --target thumbv7m-none-eabihf` 编译。
