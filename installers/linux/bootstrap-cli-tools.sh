#!/usr/bin/env bash

set -euo pipefail

if command -v apt-get >/dev/null 2>&1; then
  sudo apt-get update
  sudo apt-get install -y ffmpeg ripgrep rsync tmux zsh neovim curl git
  exit 0
fi

if command -v dnf >/dev/null 2>&1; then
  sudo dnf install -y ffmpeg ripgrep rsync tmux zsh neovim curl git
  exit 0
fi

if command -v pacman >/dev/null 2>&1; then
  sudo pacman -Syu --noconfirm ffmpeg ripgrep rsync tmux zsh neovim curl git
  exit 0
fi

echo "No supported package manager was found. Add your package manager here."
exit 1
