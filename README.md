# my-dev-env-restorer

This repository is now organized around two jobs:

1. Keep config files in one place and sync them between the repo and a machine.
2. Keep a curated list of program download pages that can be opened in bulk.

The Rust CLI is the main entrypoint. It intentionally does not try to scrape installers or auto-download binaries, because that becomes fragile fast.

## Commands

From the repository root:

```bash
cargo run -- --configs
```

Copies the repo-managed configs to their OS-specific destinations.

```bash
cargo run -- --pull
```

Pulls local config changes back into the repo for plain copied files. Generated layered configs are skipped because reverse-merging a finished file back into shared plus fragments is not safe to do automatically.

```bash
cargo run -- --links
```

Opens the program download pages for the current platform in your browser.

```bash
cargo run -- --configs --dry-run
```

Shows what would happen without copying files or opening links.

## Layout

```txt
configs/
  shared/      # default sources used across platforms
  windows/     # only the files that truly need Windows-specific overrides
  android/     # fragment-only tweaks for Android/Termux
manifests/
  configs.toml # config targets + source selection
  programs.toml
installers/
  linux/
  macos/
Scripts/
  ...          # personal utility scripts kept as-is
```

## Shared config model

Each config entry can point to:

- `shared`: the default full file used everywhere
- `overlays.<platform>`: a full-file override when a platform needs a completely different version
- `fragments`: shared snippets appended after the base file
- `platform_fragments.<platform>`: platform-only snippets appended after the base file

The CLI builds the final output in this order:

1. `overlays.<platform>` if present, otherwise `shared`
2. any `fragments`
3. any `platform_fragments.<platform>`

That gives you one main config file and small per-platform tweaks instead of duplicated whole files.

## Adding a new config

1. Put the default file in `configs/shared/...`.
2. Add a full overlay only if a platform truly needs a different whole file.
3. Prefer small fragment files for minor differences such as aliases, hotkeys, shell paths, or platform-specific commands.
4. Register the source files and targets in `manifests/configs.toml`.

## Example

```toml
[[config]]
id = "vimrc"
shared = "configs/shared/vim/.vimrc"
platform_fragments = { android = ["configs/android/vim/.vimrc.fragment"] }
targets = { linux = "~/.vimrc", android = "~/.vimrc" }
```

With that setup, the Android target is generated from the shared `.vimrc` plus the Android fragment.

## Adding a new program download page

Add another `[[program]]` entry to `manifests/programs.toml`.

## Unix CLI bootstrap helpers

There are small package-manager-driven install helpers for common terminal tools:

- [installers/linux/bootstrap-cli-tools.sh](/D:/projects/node/my-dev-env-restorer/installers/linux/bootstrap-cli-tools.sh)
- [installers/macos/bootstrap-cli-tools.sh](/D:/projects/node/my-dev-env-restorer/installers/macos/bootstrap-cli-tools.sh)

They currently install:

- `ffmpeg`
- `ripgrep`
- `rsync`
- `tmux`
- `zsh`
- `neovim`
- `curl`
- `git`

## Notes

- `configs/shared/vscode/extensions.txt` is kept as a tracked list of useful VS Code extensions.
- Standalone utility folders such as `Scripts/` and `Windows/prog/` are still here; this migration is focused on environment restoration, not deleting your personal tooling.
