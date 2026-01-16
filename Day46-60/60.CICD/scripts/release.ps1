# Release script for semantic versioning (Windows PowerShell)

param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

$ErrorActionPreference = "Stop"

# Validate semantic version format
if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    Write-Host "❌ Error: Invalid version format" -ForegroundColor Red
    Write-Host "Version must be in format: MAJOR.MINOR.PATCH (e.g., 0.2.0)" -ForegroundColor Yellow
    exit 1
}

Write-Host "🚀 Starting release process for v$Version" -ForegroundColor Green

# Check if working directory is clean
$status = git status --porcelain
if ($status) {
    Write-Host "⚠️  Warning: Working directory is not clean" -ForegroundColor Yellow
    git status --short
    $response = Read-Host "Continue anyway? (y/n)"
    if ($response -ne 'y') {
        exit 1
    }
}

# Update version in Cargo.toml
Write-Host "`n📝 Updating Cargo.toml version..." -ForegroundColor Cyan
$cargoToml = Get-Content "Cargo.toml" -Raw
$cargoToml = $cargoToml -replace 'version = ".*?"', "version = `"$Version`""
Set-Content "Cargo.toml" -Value $cargoToml

# Generate changelog (requires git-cliff)
if (Get-Command git-cliff -ErrorAction SilentlyContinue) {
    Write-Host "📋 Generating CHANGELOG..." -ForegroundColor Cyan
    git-cliff -o CHANGELOG.md
} else {
    Write-Host "⚠️  Warning: git-cliff not found, skipping changelog generation" -ForegroundColor Yellow
}

# Run tests
Write-Host "`n🧪 Running tests..." -ForegroundColor Cyan
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tests failed!" -ForegroundColor Red
    exit 1
}

# Commit changes
Write-Host "`n💾 Committing changes..." -ForegroundColor Cyan
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release v$Version"

# Create tag
Write-Host "🏷️  Creating tag v$Version..." -ForegroundColor Cyan
git tag -a "v$Version" -m "Release v$Version"

Write-Host "`n✅ Release prepared successfully!" -ForegroundColor Green
Write-Host "To complete the release, run:" -ForegroundColor Yellow
Write-Host "  git push origin main --tags" -ForegroundColor White
