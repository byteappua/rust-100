#!/bin/bash
# 完整的项目检查脚本

set -e

echo "🚀 开始完整项目检查..."
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查函数
check_step() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ $1 通过${NC}"
    else
        echo -e "${RED}❌ $1 失败${NC}"
        exit 1
    fi
}

# 1. 格式检查
echo "📝 检查代码格式..."
cargo fmt --all -- --check
check_step "代码格式"
echo ""

# 2. Clippy 检查
echo "🔍 运行 Clippy 检查..."
cargo clippy --all-features -- -D warnings
check_step "Clippy 检查"
echo ""

# 3. 编译检查
echo "🔨 编译项目..."
cargo check --all-features
check_step "编译检查"
echo ""

# 4. 构建项目
echo "🏗️  构建项目（debug）..."
cargo build
check_step "Debug 构建"
echo ""

echo "🏗️  构建项目（release）..."
cargo build --release
check_step "Release 构建"
echo ""

# 5. 运行测试
echo "🧪 运行单元测试..."
cargo test --lib
check_step "单元测试"
echo ""

echo "🧪 运行集成测试..."
cargo test --test '*'
check_step "集成测试"
echo ""

echo "🧪 运行文档测试..."
cargo test --doc
check_step "文档测试"
echo ""

# 6. 生成文档
echo "📚 生成文档..."
cargo doc --no-deps --all-features
check_step "文档生成"
echo ""

# 7. 运行示例
echo "🎯 运行示例程序..."
cargo run --example version_check
check_step "示例程序"
echo ""

# 8. 运行主程序
echo "🎯 运行主程序..."
cargo run
check_step "主程序"
echo ""

# 9. 运行基准测试（可选）
if command -v cargo-criterion &> /dev/null; then
    echo "⚡ 运行性能测试..."
    cargo bench --no-fail-fast
    check_step "性能测试"
    echo ""
fi

# 10. 安全审计（可选）
if command -v cargo-audit &> /dev/null; then
    echo "🔒 运行安全审计..."
    cargo audit
    check_step "安全审计"
    echo ""
fi

echo ""
echo -e "${GREEN}🎉 所有检查通过！项目状态良好。${NC}"
echo ""
echo "📊 项目统计:"
echo "  - 源文件: $(find src -name '*.rs' | wc -l)"
echo "  - 测试文件: $(find tests -name '*.rs' 2>/dev/null | wc -l)"
echo "  - 示例文件: $(find examples -name '*.rs' 2>/dev/null | wc -l)"
echo "  - 代码行数: $(find src -name '*.rs' -exec cat {} \; | wc -l)"
echo ""
