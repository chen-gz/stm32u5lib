# 使用 Debian Bookworm 作为基础镜像
FROM debian:bookworm

# 安装必要工具和依赖
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    sudo \
    pkg-config \
    libusb-1.0-0-dev \
    libudev-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# 创建 vscode 用户
RUN groupadd --gid 1000 vscode \
    && useradd --uid 1000 --gid vscode --shell /bin/bash --create-home vscode \
    && echo 'vscode ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# 切换到 vscode 用户
USER vscode
WORKDIR /home/vscode

# 安装 rustup 和指定 Rust 版本
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.88.0

# 添加 rustup cargo bin 到 PATH
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

# 验证安装
RUN rustc --version && cargo --version

# 设置 Rust 环境变量
ENV RUST_BACKTRACE=1

