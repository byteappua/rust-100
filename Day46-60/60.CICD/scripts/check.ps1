# Pre-commit checks script for Windows PowerShell

$ErrorActionPreference = "Stop"

Write-Host "🔍 Running pre-commit checks..." -ForegroundColor Cyan

# Check formatting
Write-Host "`n📝 Checking code formatting..." -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Formatting check failed!" -ForegroundColor Red
    exit 1
}

# Run clippy
Write-Host "`n📎 Running clippy..." -ForegroundColor Yellow
cargo clippy --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Clippy check failed!" -ForegroundColor Red
    exit 1
}

# Run tests
Write-Host "`n🧪 Running tests..." -ForegroundColor Yellow
cargo test --all-features
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tests failed!" -ForegroundColor Red
    exit 1
}

# Check documentation
Write-Host "`n📚 Checking documentation..." -ForegroundColor Yellow
cargo doc --no-deps --all-features
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Documentation check failed!" -ForegroundColor Red
    exit 1
}

Write-Host "`n✅ All checks passed!" -ForegroundColor Green
