.PHONY: help build test check fmt clippy doc clean run release install bench coverage

# 默认目标
help:
	@echo "可用的 Make 命令:"
	@echo "  make build      - 构建项目（debug 模式）"
	@echo "  make release    - 构建项目（release 模式）"
	@echo "  make test       - 运行所有测试"
	@echo "  make check      - 运行所有检查（fmt + clippy + test）"
	@echo "  make fmt        - 格式化代码"
	@echo "  make clippy     - 运行 Clippy 检查"
	@echo "  make doc        - 生成文档"
	@echo "  make run        - 运行项目"
	@echo "  make clean      - 清理构建产物"
	@echo "  make install    - 安装到系统"
	@echo "  make bench      - 运行性能测试"
	@echo "  make coverage   - 生成代码覆盖率报告"

# 构建项目
build:
	cargo build

# 发布构建
release:
	cargo build --release

# 运行测试
test:
	cargo test --all-features

# 运行所有检查
check: fmt clippy test
	@echo "✅ 所有检查通过！"

# 格式化代码
fmt:
	cargo fmt --all

# 检查格式
fmt-check:
	cargo fmt --all -- --check

# 运行 Clippy
clippy:
	cargo clippy --all-features -- -D warnings

# 生成文档
doc:
	cargo doc --no-deps --all-features --open

# 生成文档（不打开）
doc-build:
	cargo doc --no-deps --all-features

# 运行项目
run:
	cargo run

# 运行示例
example:
	cargo run --example version_check

# 清理构建产物
clean:
	cargo clean

# 安装到系统
install:
	cargo install --path .

# 运行性能测试
bench:
	cargo bench

# 生成代码覆盖率
coverage:
	cargo tarpaulin --out Html --output-dir coverage

# 更新依赖
update:
	cargo update

# 审计依赖安全性
audit:
	cargo audit

# 构建 Docker 镜像
docker-build:
	docker build -t mini-redis-cicd:latest .

# 运行 Docker 容器
docker-run:
	docker run --rm mini-redis-cicd:latest

# 使用 Docker Compose
docker-up:
	docker-compose up

docker-down:
	docker-compose down

# 发布准备
pre-release: check doc-build
	@echo "✅ 准备发布..."

# 完整的 CI 检查
ci: fmt-check clippy test doc-build
	@echo "✅ CI 检查完成！"
