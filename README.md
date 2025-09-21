# zls

> ⚡ A blazingly fast `ls` replacement written in Rust

**z**uedev's **ls** is a modern replacement for the `ls` command with a focus on speed and simplicity. It leverages parallel directory reading and provides colorized output with intuitive file type indicators.

## Features

- 🚀 **Fast directory listings** - Parallel processing for improved performance ([`src/main.rs:113`](src/main.rs#L113))
- 🎨 **Colorized output** - File type indicators and syntax highlighting ([`src/main.rs:148-156`](src/main.rs#L148-L156), [`src/main.rs:163-174`](src/main.rs#L163-L174))
- 📋 **Detailed by default** - Long format with file sizes and timestamps ([`src/main.rs:142-159`](src/main.rs#L142-L159))
- 🔍 **Flexible sorting** - Sort by name or modification time ([`src/main.rs:126-134`](src/main.rs#L126-L134))
- 👁️ **Hidden file support** - Show/hide dotfiles with `-a` flag ([`src/main.rs:122-124`](src/main.rs#L122-L124))
- 📏 **Human-readable sizes** - Convert bytes to KB/MB/GB with `-H` flag ([`src/main.rs:78-97`](src/main.rs#L78-L97))
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
