# 使用官方的 Rust 镜像作为基础镜像
FROM rust:1.82 as builder

# 创建一个新的空目录，作为我们的工作空间
WORKDIR /usr/src/app

# 复制 Cargo.toml 和 Cargo.lock（如果存在），并创建依赖缓存层
COPY Cargo.toml Cargo.lock ./

# 将源代码复制到工作目录
COPY . .

# 为了更快地构建依赖，先将所有的依赖项拉取到临时的主文件
RUN cargo fetch



# 构建二进制文件为可执行文件
RUN cargo build --release

# 使用更小的基础镜像来减小最终镜像的体积
FROM ubuntu:22.04

# 定义服务在 Docker 中运行的端口
EXPOSE 5001

# 创建一个新的空目录，作为应用程序所在的目录
WORKDIR /usr/local/bin

# 从构建阶段拷贝编译好的二进制文件
COPY --from=builder /usr/src/app/target/release/MockTrading .

# 定义默认启动命令
CMD ["./MockTrading"]
