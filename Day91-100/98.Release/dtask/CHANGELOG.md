# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-01-16

### Added
- Core distributed task scheduling system.
- gRPC API for task submission and management.
- REST API (Axum) for task management and status.
- SQLite storage with `sqlx`.
- JWT Authentication.
- Prometheus Metrics.
- OpenRaft integration (stubbed for future extension).
- Docker deployment support.
- CI/CD workflow configuration.

### Fixed
- Resolved gRPC and Web API concurrency issues.
- Optimized binary size with release profile.
