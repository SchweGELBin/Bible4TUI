# Bible4TUI
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/SchweGELBin/Bible4TUI/total)](https://github.com/SchweGELBin/Bible4TUI/releases)
[![GitHub License](https://img.shields.io/github/license/SchweGELBin/Bible4TUI)](../LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/SchweGELBin/Bible4TUI)](https://github.com/SchweGELBin/Bible4TUI/releases/latest)

- Bible program for the terminal
- Available in my [nur expressions](https://github.com/SchweGELBin/nur-expressions) repo
- This program is very experimental and should not be used in the current state

## Build

### General
``` bash
git clone https://github.com/SchweGELBin/Bible4TUI.git
cd Bible4TUI
cargo build --release
```
Binary will be at `./target/release/bible4tui`

### Nix
``` bash
nix build github:SchweGELBin/Bible4TUI
```
Binary will be at `./result/bin/bible4tui`
