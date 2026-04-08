#!/usr/bin/env bash

set -euo pipefail

if ! command -v brew >/dev/null 2>&1; then
  echo "Homebrew is required. Install it from https://brew.sh/ first."
  exit 1
fi

brew install ffmpeg ripgrep rsync tmux zsh neovim curl git
