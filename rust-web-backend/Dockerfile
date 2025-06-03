# 医生投放评价系统 - Docker 构建配置
# 多阶段构建，优化镜像大小

# 构建阶段
FROM rust:1.75-slim as builder

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件并预构建依赖（缓存优化）
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 复制源代码
COPY src ./src
COPY migrations ./migrations

# 构建应用
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 创建应用用户
RUN useradd -r -s /bin/false rustapp

# 设置工作目录
WORKDIR /app

# 复制构建产物
COPY --from=builder /app/target/release/rust-web-backend ./rust-web-backend
COPY --from=builder /app/migrations ./migrations

# 复制前端文件
COPY frontend ./frontend

# 创建数据目录
RUN mkdir -p data logs && chown rustapp:rustapp data logs

# 设置文件权限
RUN chmod +x rust-web-backend

# 切换到应用用户
USER rustapp

# 暴露端口
EXPOSE 8081

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/api/health || exit 1

# 设置环境变量
ENV RUST_LOG=info
ENV RUST_WEB_PORT=8081
ENV RUST_WEB_HOST=0.0.0.0
ENV DATABASE_URL=sqlite:./data/doctor_analysis.db

# 启动应用
CMD ["./rust-web-backend"]
