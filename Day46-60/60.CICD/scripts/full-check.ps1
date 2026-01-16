# 完整的项目检查脚本 (PowerShell)

$ErrorActionPreference = "Stop"

Write-Host "🚀 开始完整项目检查..." -ForegroundColor Cyan
Write-Host ""

# 检查函数
function Check-Step {
    param($Name)
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ $Name 通过" -ForegroundColor Green
    } else {
        Write-Host "❌ $Name 失败" -ForegroundColor Red
        exit 1
    }
}

# 1. 格式检查
Write-Host "📝 检查代码格式..." -ForegroundColor Yellow
cargo fmt --all -- --check
Check-Step "代码格式"
Write-Host ""

# 2. Clippy 检查
Write-Host "🔍 运行 Clippy 检查..." -ForegroundColor Yellow
cargo clippy --all-features -- -D warnings
Check-Step "Clippy 检查"
Write-Host ""

# 3. 编译检查
Write-Host "🔨 编译项目..." -ForegroundColor Yellow
cargo check --all-features
Check-Step "编译检查"
Write-Host ""

# 4. 构建项目
Write-Host "🏗️  构建项目（debug）..." -ForegroundColor Yellow
cargo build
Check-Step "Debug 构建"
Write-Host ""

Write-Host "🏗️  构建项目（release）..." -ForegroundColor Yellow
cargo build --release
Check-Step "Release 构建"
Write-Host ""

# 5. 运行测试
Write-Host "🧪 运行单元测试..." -ForegroundColor Yellow
cargo test --lib
Check-Step "单元测试"
Write-Host ""

Write-Host "🧪 运行集成测试..." -ForegroundColor Yellow
cargo test --test '*'
Check-Step "集成测试"
Write-Host ""

Write-Host "🧪 运行文档测试..." -ForegroundColor Yellow
cargo test --doc
Check-Step "文档测试"
Write-Host ""

# 6. 生成文档
Write-Host "📚 生成文档..." -ForegroundColor Yellow
cargo doc --no-deps --all-features
Check-Step "文档生成"
Write-Host ""

# 7. 运行示例
Write-Host "🎯 运行示例程序..." -ForegroundColor Yellow
cargo run --example version_check
Check-Step "示例程序"
Write-Host ""

# 8. 运行主程序
Write-Host "🎯 运行主程序..." -ForegroundColor Yellow
cargo run
Check-Step "主程序"
Write-Host ""

# 9. 运行基准测试（可选）
if (Get-Command cargo-criterion -ErrorAction SilentlyContinue) {
    Write-Host "⚡ 运行性能测试..." -ForegroundColor Yellow
    cargo bench --no-fail-fast
    Check-Step "性能测试"
    Write-Host ""
}

# 10. 安全审计（可选）
if (Get-Command cargo-audit -ErrorAction SilentlyContinue) {
    Write-Host "🔒 运行安全审计..." -ForegroundColor Yellow
    cargo audit
    Check-Step "安全审计"
    Write-Host ""
}

Write-Host ""
Write-Host "🎉 所有检查通过！项目状态良好。" -ForegroundColor Green
Write-Host ""
Write-Host "📊 项目统计:" -ForegroundColor Cyan
$srcFiles = (Get-ChildItem -Path src -Filter *.rs -Recurse).Count
$testFiles = (Get-ChildItem -Path tests -Filter *.rs -Recurse -ErrorAction SilentlyContinue).Count
$exampleFiles = (Get-ChildItem -Path examples -Filter *.rs -Recurse -ErrorAction SilentlyContinue).Count
$codeLines = (Get-Content -Path (Get-ChildItem -Path src -Filter *.rs -Recurse) | Measure-Object -Line).Lines

Write-Host "  - 源文件: $srcFiles"
Write-Host "  - 测试文件: $testFiles"
Write-Host "  - 示例文件: $exampleFiles"
Write-Host "  - 代码行数: $codeLines"
Write-Host ""
