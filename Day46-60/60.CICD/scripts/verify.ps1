# 项目验证脚本 - 运行所有检查

$ErrorActionPreference = "Stop"

Write-Host "🔍 开始项目验证..." -ForegroundColor Cyan
Write-Host ""

$failed = $false

# 1. 格式检查
Write-Host "1️⃣  检查代码格式..." -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 格式检查失败" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "✅ 格式检查通过" -ForegroundColor Green
}
Write-Host ""

# 2. Clippy 检查
Write-Host "2️⃣  运行 Clippy..." -ForegroundColor Yellow
cargo clippy --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Clippy 检查失败" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "✅ Clippy 检查通过" -ForegroundColor Green
}
Write-Host ""

# 3. 编译检查
Write-Host "3️⃣  编译项目..." -ForegroundColor Yellow
cargo build --all-features
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 编译失败" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "✅ 编译成功" -ForegroundColor Green
}
Write-Host ""

# 4. 运行测试
Write-Host "4️⃣  运行测试..." -ForegroundColor Yellow
cargo test --all-features
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 测试失败" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "✅ 测试通过" -ForegroundColor Green
}
Write-Host ""

# 5. 生成文档
Write-Host "5️⃣  生成文档..." -ForegroundColor Yellow
cargo doc --no-deps --all-features
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 文档生成失败" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "✅ 文档生成成功" -ForegroundColor Green
}
Write-Host ""

# 6. 运行示例
Write-Host "6️⃣  运行示例程序..." -ForegroundColor Yellow
cargo run --example version_check > $null 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 示例运行失败" -ForegroundColor Red
    $failed = $true
} else {
    Write-Host "✅ 示例运行成功" -ForegroundColor Green
}
Write-Host ""

# 总结
Write-Host "=" * 50 -ForegroundColor Cyan
if ($failed) {
    Write-Host "❌ 验证失败！请修复上述问题。" -ForegroundColor Red
    exit 1
} else {
    Write-Host "✅ 所有检查通过！项目状态良好。" -ForegroundColor Green
    Write-Host ""
    Write-Host "📊 项目统计:" -ForegroundColor Cyan
    Write-Host "  - 源文件: $(Get-ChildItem -Path src -Filter *.rs -Recurse | Measure-Object).Count"
    Write-Host "  - 测试文件: $(Get-ChildItem -Path tests -Filter *.rs -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count"
    Write-Host "  - 示例文件: $(Get-ChildItem -Path examples -Filter *.rs -Recurse -ErrorAction SilentlyContinue | Measure-Object).Count"
    Write-Host "  - 文档文件: $(Get-ChildItem -Filter *.md -Recurse | Measure-Object).Count"
    exit 0
}
