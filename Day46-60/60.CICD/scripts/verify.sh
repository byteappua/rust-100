#!/bin/bash
# 项目验证脚本 - 运行所有检查

set -e

echo "🔍 开始项目验证..."
echo ""

failed=false

# 1. 格式检查
echo "1️⃣  检查代码格式..."
if cargo fmt --all -- --check; then
    echo "✅ 格式检查通过"
else
    echo "❌ 格式检查失败"
    failed=true
fi
echo ""

# 2. Clippy 检查
echo "2️⃣  运行 Clippy..."
if cargo clippy --all-features -- -D warnings; then
    echo "✅ Clippy 检查通过"
else
    echo "❌ Clippy 检查失败"
    failed=true
fi
echo ""

# 3. 编译检查
echo "3️⃣  编译项目..."
if cargo build --all-features; then
    echo "✅ 编译成功"
else
    echo "❌ 编译失败"
    failed=true
fi
echo ""

# 4. 运行测试
echo "4️⃣  运行测试..."
if cargo test --all-features; then
    echo "✅ 测试通过"
else
    echo "❌ 测试失败"
    failed=true
fi
echo ""

# 5. 生成文档
echo "5️⃣  生成文档..."
if cargo doc --no-deps --all-features; then
    echo "✅ 文档生成成功"
else
    echo "❌ 文档生成失败"
    failed=true
fi
echo ""

# 6. 运行示例
echo "6️⃣  运行示例程序..."
if cargo run --example version_check > /dev/null 2>&1; then
    echo "✅ 示例运行成功"
else
    echo "❌ 示例运行失败"
    failed=true
fi
echo ""

# 总结
echo "=================================================="
if [ "$failed" = true ]; then
    echo "❌ 验证失败！请修复上述问题。"
    exit 1
else
    echo "✅ 所有检查通过！项目状态良好。"
    echo ""
    echo "📊 项目统计:"
    echo "  - 源文件: $(find src -name '*.rs' | wc -l)"
    echo "  - 测试文件: $(find tests -name '*.rs' 2>/dev/null | wc -l)"
    echo "  - 示例文件: $(find examples -name '*.rs' 2>/dev/null | wc -l)"
    echo "  - 文档文件: $(find . -name '*.md' | wc -l)"
    exit 0
fi
