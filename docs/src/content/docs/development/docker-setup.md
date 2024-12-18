---
title: Docker setup
description: Docker setup guide.
---

# Clean Rootless Docker Installation Guide

## Uninstall Existing Docker (if any)

```bash
# Stop all Docker services
sudo systemctl stop docker.socket
sudo systemctl stop docker
sudo systemctl stop containerd

# Remove packages
sudo apt-get remove -y \
    docker \
    docker-engine \
    docker.io \
    containerd \
    runc \
    docker-ce \
    docker-ce-cli \
    containerd.io \
    docker-buildx-plugin \
    docker-compose-plugin

# Clean up all Docker files
sudo rm -rf /var/lib/docker
sudo rm -rf /var/lib/containerd
sudo rm -rf /etc/docker
sudo rm -rf /var/run/docker.sock
sudo rm -rf ~/.docker

# Remove the Docker group
sudo groupdel docker

# Clean up packages
sudo apt-get autoremove -y
```

## Install Rootless Docker

```bash
# Install prerequisites
sudo apt update
sudo apt install -y \
    uidmap \
    dbus-user-session

# Install rootless Docker
curl -fsSL https://get.docker.com/rootless | sh
```

### For bash/zsh users
Add to `~/.bashrc` or `~/.zshrc`:
```bash
export PATH=/home/$USER/bin:$PATH
export DOCKER_HOST=unix:///run/user/$UID/docker.sock

# Apply changes
source ~/.bashrc  # or source ~/.zshrc
```

### For fish users
Add to `~/.config/fish/config.fish`:
```fish
set -x PATH /home/$USER/bin $PATH
set -x DOCKER_HOST unix:///run/user/(id -u)/docker.sock

# Apply changes
source ~/.config/fish/config.fish
```

## Start Docker Service

```bash
# Start Docker service
systemctl --user start docker
systemctl --user enable docker
```

## Verify Installation

```bash
# Check if Docker is running
systemctl --user status docker

# Run test container
docker run hello-world

# Verify it's running rootless
docker info | grep -i rootless
```

## Service Management

```bash
# Start Docker
systemctl --user start docker

# Stop Docker
systemctl --user stop docker

# Restart Docker
systemctl --user restart docker

# Check status
systemctl --user status docker

# View logs
journalctl --user -u docker
```
