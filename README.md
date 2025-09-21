# zls

> ⚡ A blazingly fast `ls` replacement written in Rust

**z**uedev's **ls** is a modern replacement for the `ls` command with a focus on speed and simplicity. It leverages parallel directory reading and provides colorized output with intuitive file type indicators.

## Features

- 🚀 **Fast directory listings** - Parallel processing for improved performance
- 🎨 **Colorized output** - File type indicators and syntax highlighting
- 📋 **Detailed by default** - Long format with file sizes and timestamps
- 🔍 **Flexible sorting** - Sort by name or modification time
- 👁️ **Hidden file support** - Show/hide dotfiles with `-a` flag
- 📏 **Human-readable sizes** - Convert bytes to KB/MB/GB with `-H` flag
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
    -s, --short     Use short listing format
    -t, --time      Sort by modification time
    -H, --human     Show human readable sizes
```
