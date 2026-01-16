# 贡献指南

感谢你对 mini-redis-cicd 项目的关注！我们欢迎各种形式的贡献。

## 行为准则

请遵守我们的行为准则，保持友好和尊重的交流环境。

## 如何贡献

### 报告 Bug

如果你发现了 bug，请创建一个 issue，包含以下信息：

- 清晰的标题和描述
- 重现步骤
- 预期行为和实际行为
- 环境信息（操作系统、Rust 版本等）
- 相关的代码片段或错误信息

### 提出新功能

如果你有新功能的想法：

1. 先创建一个 issue 讨论这个功能
2. 说明为什么需要这个功能
3. 描述你期望的行为
4. 等待维护者的反馈

### 提交代码

#### 开发流程

1. **Fork 项目**
   ```bash
   # 在 GitHub 上 fork 项目
   git clone https://github.com/your-username/mini-redis-cicd.git
   cd mini-redis-cicd
   ```

2. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   # 或
   git checkout -b fix/your-bug-fix
   ```

3. **进行开发**
   - 编写代码
   - 添加测试
   - 更新文档

4. **运行检查**
   ```bash
   # 使用 Make
   make check

   # 或手动运行
   cargo fmt --all
   cargo clippy --all-features -- -D warnings
   cargo test --all-features
   cargo doc --no-deps --all-features
   ```

5. **提交代码**
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

6. **推送并创建 PR**
   ```bash
   git push origin feature/your-feature-name
   # 然后在 GitHub 上创建 Pull Request
   ```

#### 提交信息规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**类型（type）：**
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响代码运行）
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

**示例：**
```
feat(version): add version comparison function

Add a new function to compare semantic versions.
This function returns an Ordering enum to indicate
which version is greater.

Closes #123
```

#### 代码规范

1. **格式化**
   - 使用 `cargo fmt` 格式化代码
   - 遵循 Rust 官方风格指南

2. **Clippy**
   - 修复所有 clippy 警告
   - 运行 `cargo clippy -- -D warnings`

3. **测试**
   - 为新功能添加单元测试
   - 为公共 API 添加文档测试
   - 确保测试覆盖率不降低

4. **文档**
   - 为公共 API 添加文档注释
   - 包含示例代码
   - 更新 README 和相关文档

5. **性能**
   - 避免不必要的分配
   - 使用基准测试验证性能改进

#### 代码审查

所有的 PR 都需要经过代码审查：

- 至少一个维护者的批准
- 所有 CI 检查通过
- 没有未解决的评论

### 文档贡献

文档同样重要！你可以：

- 修复文档中的错误
- 改进现有文档
- 添加新的示例
- 翻译文档

## 开发环境设置

### 必需工具

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装开发工具
cargo install cargo-watch
cargo install cargo-tarpaulin
cargo install cargo-audit
cargo install git-cliff
```

### 推荐工具

```bash
# 代码覆盖率
cargo install cargo-tarpaulin

# 安全审计
cargo install cargo-audit

# 依赖更新检查
cargo install cargo-outdated

# 许可证检查
cargo install cargo-license
```

### IDE 配置

推荐使用 VS Code 配合以下插件：

- rust-analyzer
- Even Better TOML
- crates
- CodeLLDB

配置文件已包含在 `.vscode/` 目录中。

## 测试

### 运行测试

```bash
# 所有测试
cargo test

# 特定测试
cargo test test_name

# 显示输出
cargo test -- --nocapture

# 文档测试
cargo test --doc
```

### 编写测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

## 发布流程

只有维护者可以发布新版本：

1. 更新版本号
2. 生成 CHANGELOG
3. 创建 tag
4. 推送触发 CI/CD

详见 `scripts/release.sh`。

## 获取帮助

如果你有任何问题：

- 查看 [README.md](README.md)
- 查看 [QUICKSTART.md](QUICKSTART.md)
- 创建一个 issue
- 加入我们的讨论区

## 许可证

通过贡献代码，你同意你的贡献将在 MIT 或 Apache-2.0 许可证下发布。

---

再次感谢你的贡献！🎉
