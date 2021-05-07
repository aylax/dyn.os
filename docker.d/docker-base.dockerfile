# ------------------------------------------------------------------------------
# Init Basic Env Stage
# ------------------------------------------------------------------------------
FROM rust:1.50 as cargo-build
RUN  sed -i s@/deb.debian.org/@/mirrors.ustc.edu.cn/@g /etc/apt/sources.list
RUN apt-get clean && \
    apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app/lynn

