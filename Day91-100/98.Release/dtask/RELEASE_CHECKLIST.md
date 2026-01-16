# Release Checklist

## Pre-Release
- [ ] **Tests Pass**: Run `cargo test` and ensure all tests pass.
- [ ] **Lints Check**: Run `cargo clippy` and address warnings.
- [ ] **Format Check**: Run `cargo fmt --check`.
- [ ] **Documentation**: Ensure `cargo doc` generates without errors.
- [ ] **Dependency Audit**: Run `cargo audit` (if available) or check for vulnerable dependencies.

## Versioning
- [ ] **Update Cargo.toml**: Bump version number (e.g., `1.0.0`).
- [ ] **Update CHANGELOG.md**: Add entry for the new version.
- [ ] **Git Tag**: Create a git tag (e.g., `v1.0.0`).

## Build
- [ ] **Clean Build**: `cargo clean && cargo build --release`.
- [ ] **Docker Build**: `docker build -t dtask:1.0.0 .`.
- [ ] **Docker Run**: Verify the image runs correctly locally.

## Deployment
- [ ] **Push Image**: Push Docker image to registry (e.g., Docker Hub, GHCR).
- [ ] **Deploy**: Update deployment manifests (k8s, compose) with new tag.
- [ ] **Verify**: Check logs and metrics after deployment.
