# 部署指南

本文档介绍如何将 Mini-Redis CI/CD 项目部署到各种环境。

## 目录

- [本地部署](#本地部署)
- [Docker 部署](#docker-部署)
- [Kubernetes 部署](#kubernetes-部署)
- [云平台部署](#云平台部署)
- [生产环境最佳实践](#生产环境最佳实践)

## 本地部署

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/yourusername/mini-redis-cicd.git
cd mini-redis-cicd

# 构建发布版本
cargo build --release

# 运行
./target/release/mini-redis-cicd
```

### 使用 Cargo Install

```bash
# 从 crates.io 安装
cargo install mini-redis-cicd

# 运行
mini-redis-cicd
```

## Docker 部署

### 使用预构建镜像

```bash
# 拉取镜像
docker pull ghcr.io/yourusername/mini-redis-cicd:latest

# 运行容器
docker run -d \
  --name mini-redis \
  -p 6379:6379 \
  ghcr.io/yourusername/mini-redis-cicd:latest
```

### 从源码构建镜像

```bash
# 构建镜像
docker build -t mini-redis-cicd:latest .

# 运行容器
docker run -d \
  --name mini-redis \
  -p 6379:6379 \
  mini-redis-cicd:latest
```

### 使用 Docker Compose

创建 `docker-compose.yml`:

```yaml
version: '3.8'

services:
  mini-redis:
    image: mini-redis-cicd:latest
    container_name: mini-redis
    ports:
      - "6379:6379"
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "mini-redis-cicd", "--health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

运行：

```bash
docker-compose up -d
```

## Kubernetes 部署

### 创建 Deployment

创建 `k8s/deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mini-redis
  labels:
    app: mini-redis
spec:
  replicas: 3
  selector:
    matchLabels:
      app: mini-redis
  template:
    metadata:
      labels:
        app: mini-redis
    spec:
      containers:
      - name: mini-redis
        image: ghcr.io/yourusername/mini-redis-cicd:latest
        ports:
        - containerPort: 6379
        resources:
          requests:
            memory: "64Mi"
            cpu: "250m"
          limits:
            memory: "128Mi"
            cpu: "500m"
        livenessProbe:
          tcpSocket:
            port: 6379
          initialDelaySeconds: 15
          periodSeconds: 20
        readinessProbe:
          tcpSocket:
            port: 6379
          initialDelaySeconds: 5
          periodSeconds: 10
```

### 创建 Service

创建 `k8s/service.yaml`:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: mini-redis
spec:
  selector:
    app: mini-redis
  ports:
  - protocol: TCP
    port: 6379
    targetPort: 6379
  type: LoadBalancer
```

### 部署到集群

```bash
# 应用配置
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# 检查状态
kubectl get pods
kubectl get services

# 查看日志
kubectl logs -f deployment/mini-redis
```

## 云平台部署

### AWS ECS

1. 创建 ECR 仓库：
```bash
aws ecr create-repository --repository-name mini-redis-cicd
```

2. 推送镜像：
```bash
aws ecr get-login-password --region us-east-1 | \
  docker login --username AWS --password-stdin <account-id>.dkr.ecr.us-east-1.amazonaws.com

docker tag mini-redis-cicd:latest <account-id>.dkr.ecr.us-east-1.amazonaws.com/mini-redis-cicd:latest
docker push <account-id>.dkr.ecr.us-east-1.amazonaws.com/mini-redis-cicd:latest
```

3. 创建 ECS 任务定义和服务（使用 AWS Console 或 CLI）

### Google Cloud Run

```bash
# 构建并推送到 GCR
gcloud builds submit --tag gcr.io/PROJECT-ID/mini-redis-cicd

# 部署到 Cloud Run
gcloud run deploy mini-redis \
  --image gcr.io/PROJECT-ID/mini-redis-cicd \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated
```

### Azure Container Instances

```bash
# 创建资源组
az group create --name mini-redis-rg --location eastus

# 创建容器实例
az container create \
  --resource-group mini-redis-rg \
  --name mini-redis \
  --image ghcr.io/yourusername/mini-redis-cicd:latest \
  --dns-name-label mini-redis-unique \
  --ports 6379
```

### Fly.io

创建 `fly.toml`:

```toml
app = "mini-redis-cicd"

[build]
  image = "ghcr.io/yourusername/mini-redis-cicd:latest"

[[services]]
  internal_port = 6379
  protocol = "tcp"

  [[services.ports]]
    port = 6379
```

部署：

```bash
fly deploy
```

## 生产环境最佳实践

### 1. 环境变量配置

```bash
# 设置环境变量
export RUST_LOG=info
export MINI_REDIS_PORT=6379
export MINI_REDIS_HOST=0.0.0.0
```

### 2. 日志管理

```bash
# 使用结构化日志
RUST_LOG=mini_redis_cicd=info,tokio=warn cargo run

# 输出到文件
cargo run 2>&1 | tee mini-redis.log
```

### 3. 监控和告警

- 使用 Prometheus 收集指标
- 配置 Grafana 仪表板
- 设置告警规则

### 4. 备份策略

```bash
# 定期备份数据
0 2 * * * /usr/local/bin/backup-mini-redis.sh
```

### 5. 安全配置

- 使用 TLS/SSL 加密连接
- 配置防火墙规则
- 限制访问 IP
- 定期更新依赖

### 6. 性能优化

```toml
# Cargo.toml 优化配置
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### 7. 健康检查

```bash
# 添加健康检查端点
curl http://localhost:6379/health
```

### 8. 滚动更新

```bash
# Kubernetes 滚动更新
kubectl set image deployment/mini-redis \
  mini-redis=ghcr.io/yourusername/mini-redis-cicd:v2.0.0

# 监控更新状态
kubectl rollout status deployment/mini-redis

# 回滚（如果需要）
kubectl rollout undo deployment/mini-redis
```

## 故障排查

### 容器无法启动

```bash
# 查看容器日志
docker logs mini-redis

# 进入容器调试
docker exec -it mini-redis sh
```

### 端口冲突

```bash
# 检查端口占用
netstat -tulpn | grep 6379

# 使用不同端口
docker run -p 6380:6379 mini-redis-cicd:latest
```

### 性能问题

```bash
# 查看资源使用
docker stats mini-redis

# 增加资源限制
docker run --memory="256m" --cpus="1.0" mini-redis-cicd:latest
```

## 扩展阅读

- [Docker 最佳实践](https://docs.docker.com/develop/dev-best-practices/)
- [Kubernetes 部署指南](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/)
- [Rust 生产环境部署](https://www.rust-lang.org/production)

---

**更新日期**: 2026-01-16
