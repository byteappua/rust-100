# Day 72: WebAssembly (WASM) 简介

## 学习目标
- 理解 WebAssembly 概念
- 掌握 Rust 到 WASM 编译
- 学习 wasm-bindgen 使用
- 实现简单的 WASM 应用

## WebAssembly 简介

WebAssembly (WASM) 是一种可以在现代浏览器中运行的新型代码格式。

### 优势
- 接近原生的性能
- 安全的沙箱执行环境
- 跨平台
- 与 JavaScript 互操作

## 环境准备

```bash
# 安装 wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 添加 WASM 目标
rustup target add wasm32-unknown-unknown
```

## 创建 WASM 项目

```bash
cargo new --lib wasm_demo
cd wasm_demo
```

```toml
# Cargo.toml
[package]
name = "wasm_demo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

## 基本示例

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 编译

```bash
wasm-pack build --target web
```

### 在 HTML 中使用

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>WASM Demo</title>
</head>
<body>
    <script type="module">
        import init, { greet, add } from './pkg/wasm_demo.js';
        
        async function run() {
            await init();
            
            console.log(greet('World'));
            console.log(add(5, 3));
        }
        
        run();
    </script>
</body>
</html>
```

## 与 JavaScript 交互

```rust
use wasm_bindgen::prelude::*;

// 调用 JavaScript 函数
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn show_alert() {
    alert("Hello from Rust!");
}

#[wasm_bindgen]
pub fn log_message(msg: &str) {
    log(msg);
}
```

## 实用示例

### 图像处理

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn grayscale(data: &mut [u8]) {
    for chunk in data.chunks_mut(4) {
        let avg = (chunk[0] as u32 + chunk[1] as u32 + chunk[2] as u32) / 3;
        chunk[0] = avg as u8;
        chunk[1] = avg as u8;
        chunk[2] = avg as u8;
    }
}
```

详细内容请参考 Day61-80/STAGE5_OVERVIEW.md。

## 练习

1. 创建计算器 WASM 模块
2. 实现图像滤镜
3. 构建游戏逻辑
4. 创建加密工具
5. 实现数据可视化

## 下一步

Day 73 将学习前端集成实战。
