# zls

> ⚡ A blazingly fast `ls` replacement written in Rust

**z**uedev's **ls** is a modern replacement for the `ls` command with a focus on speed and simplicity. It leverages parallel directory reading and provides colorized output with intuitive file type indicators.

## Features

- 🚀 **Fast directory listings** - Parallel processing for improved performance ([`src/main.rs:108`](src/main.rs#L108))
- 🎨 **Colorized output** - File type indicators and syntax highlighting ([`src/main.rs:143-151`](src/main.rs#L143-L151), [`src/main.rs:158-169`](src/main.rs#L158-L169))
- 📋 **Detailed by default** - Long format with file sizes and timestamps ([`src/main.rs:137-154`](src/main.rs#L137-L154))
- 🔍 **Flexible sorting** - Sort by name or modification time ([`src/main.rs:121-129`](src/main.rs#L121-L129))
- 👁️ **Hidden file support** - Show/hide dotfiles with `-a` flag ([`src/main.rs:117-119`](src/main.rs#L117-L119))
- 📏 **Human-readable sizes** - Convert bytes to KB/MB/GB with `-H` flag ([`src/main.rs:73-92`](src/main.rs#L73-L92))
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
