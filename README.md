# zls

> ⚡ A blazingly fast `ls` replacement written in Rust

**z**uedev's **ls** is a modern replacement for the `ls` command with a focus on speed and simplicity. It leverages parallel directory reading and provides colorized output with intuitive file type indicators.

## Features

- 🚀 **Fast directory listings** - Parallel processing for improved performance
- 🎨 **Colorized output** - File type indicators and syntax highlighting
- 📋 **Multiple display modes** - Short and long format options
- 🔍 **Flexible sorting** - Sort by name or modification time
- 👁️ **Hidden file support** - Show/hide dotfiles with `-a` flag
- 📏 **Human-readable sizes** - Convert bytes to KB/MB/GB with `-h` flag
- 🖥️ **Cross-platform** - Linux, macOS, and Windows support

## Installation

Download the latest binary from the [releases page](https://github.com/zuedev/zls/releases) or build from source:

```bash
cargo install --path .
```

## Usage

```bash
zls [OPTIONS] [PATH]

OPTIONS:
    -a, --all       Show hidden files
    -l, --long      Use long listing format
    -t, --time      Sort by modification time
    -h, --human     Show human readable sizes
```
