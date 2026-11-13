FROM archlinux:base

ARG INSTALL_NODE=false
ARG INSTALL_PYTHON=false
ARG INSTALL_GO=false

ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    SHELL=/usr/bin/zsh

RUN pacman -Syu --noconfirm --needed \
      ca-certificates curl git less openssh procps-ng sudo tar unzip which zsh \
    && pacman -Scc --noconfirm

RUN set -eux; \
    packages=""; \
    if [ "$INSTALL_NODE" = "true" ]; then packages="$packages nodejs npm"; fi; \
    if [ "$INSTALL_PYTHON" = "true" ]; then packages="$packages python python-pip"; fi; \
    if [ "$INSTALL_GO" = "true" ]; then packages="$packages go"; fi; \
    if [ -n "$packages" ]; then \
      pacman -Syu --noconfirm --needed $packages; \
      pacman -Scc --noconfirm; \
    fi

RUN useradd --create-home --uid 1000 --shell /usr/bin/zsh codespace \
    && printf 'codespace ALL=(ALL) NOPASSWD:ALL\n' > /etc/sudoers.d/codespace \
    && chmod 0440 /etc/sudoers.d/codespace \
    && mkdir -p /workspaces \
    && chown codespace:codespace /workspaces

USER codespace
WORKDIR /workspaces
