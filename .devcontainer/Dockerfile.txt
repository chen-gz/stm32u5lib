# 使用 Debian Bookworm 作为基础镜像
FROM debian:bookworm

# 安装必要工具和依赖
RUN apt-get update && apt-get install -y curl build-essential git

# 安装 rustup 和指定 Rust 版本
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.75.0

# 添加 rustup cargo bin 到 PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# 验证版本
RUN rustc --version

# 允许 Codespaces 访问
ENV USER=vscode

