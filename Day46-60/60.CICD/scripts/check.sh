#!/bin/bash
# Pre-commit checks script

set -e

echo "🔍 Running pre-commit checks..."

# Check formatting
echo "📝 Checking code formatting..."
cargo fmt --all -- --check

# Run clippy
echo "📎 Running clippy..."
cargo clippy --all-features -- -D warnings

# Run tests
echo "🧪 Running tests..."
cargo test --all-features

# Check documentation
echo "📚 Checking documentation..."
cargo doc --no-deps --all-features

echo "✅ All checks passed!"
