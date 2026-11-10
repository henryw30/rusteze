FROM archlinux:latest

RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm --needed \
    base-devel \
    git \
    curl \
    pkgconf \
    rustup \
    mesa \
    vulkan-intel \
    vulkan-icd-loader \
    vulkan-tools \
    wayland \
    libxkbcommon \
    libx11 \
    libxrandr \
    libxi \
    libxcursor \
    libxinerama \
    && pacman -Scc --noconfirm

RUN useradd -m -u 1000 -s /bin/bash vscode

USER vscode
WORKDIR /workspace

RUN rustup default stable
