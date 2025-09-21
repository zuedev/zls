# zls

> ⚡ A blazingly fast `ls` replacement written in Rust

**zls** (zuedev's ls) is a modern, high-performance replacement for the traditional `ls` command. Built with Rust for speed and reliability, it features parallel directory processing, colorized output, and intuitive file type indicators.

## Features

- 🚀 **[Parallel Processing](src/main.rs#L112-L118)** - Leverages Rayon for concurrent directory reading and improved performance
- 🎨 **Colorized Output** - Visual file type indicators with syntax highlighting
  - 📁 [Directories (blue, bold)](src/main.rs#L148-L149)
  - 🦀 [Rust files (yellow)](src/main.rs#L150-L151)
  - 👻 [Hidden files (dark grey)](src/main.rs#L152-L153)
- 📊 **[Detailed View by Default](src/main.rs#L142-L159)** - Long format displaying file size, modification time, and type
- 📏 **[Smart Size Formatting](src/main.rs#L78-L97)** - Human-readable sizes (B/K/M/G/T) by default, with raw bytes option
- 🕒 **[Flexible Sorting](src/main.rs#L126-L134)** - Sort by name (default) or modification time
- 👁️ **[Hidden File Support](src/main.rs#L122-L124)** - Show/hide dotfiles with `-a` flag
- 📱 **[Responsive Layout](src/main.rs#L178-L184)** - Adapts short format to terminal width
- 🖥️ **Cross-Platform** - Works on Linux, macOS, and Windows

## Installation

### From Source

```bash
git clone https://github.com/zuedev/zls.git
cd zls
cargo install --path .
```

### Using Docker

```bash
# Build the image
docker build -t zls .

# Run zls in a container
docker run --rm -v $(pwd):/data zls /data
```

### From GitHub Container Registry

```bash
docker pull ghcr.io/zuedev/zls:main
docker run --rm -v $(pwd):/data ghcr.io/zuedev/zls:main /data
```

## Usage

```bash
zls [OPTIONS] [PATH]
```

### Options

| Flag | Long Form   | Description                                   |
| ---- | ----------- | --------------------------------------------- |
| `-a` | `--all`     | Show hidden files (starting with `.`)         |
| `-s` | `--short`   | Use compact listing format with column layout |
| `-t` | `--time`    | Sort by modification time (newest first)      |
| `-H` | `--human`   | Show human-readable sizes (default: enabled)  |
|      | `--bytes`   | Show raw byte sizes instead of human-readable |
| `-h` | `--help`    | Print help information                        |
| `-V` | `--version` | Print version information                     |

### Examples

```bash
# List current directory (detailed view)
zls

# List with hidden files
zls -a

# Compact view sorted by modification time
zls -st

# Show raw byte sizes
zls --bytes

# List specific directory
zls /usr/local/bin
```

## Output Formats

### Detailed View (Default)

```
d      4.1K Dec 25 10:30 src/
-      1.2K Dec 25 10:25 Cargo.toml
-      8.5K Dec 25 10:29 main.rs
```

Format: `[type] [size] [modified] [name]`

- **Type**: `d` for directory, `-` for file
- **Size**: Human-readable by default (B/K/M/G/T)
- **Modified**: `MMM DD HH:MM` format
- **Name**: Color-coded by file type

### Short View (`-s`)

```
src/  Cargo.toml  main.rs  target/  README.md
```

Responsive column layout that adapts to terminal width.

## Performance

zls uses parallel processing via Rayon to read directory entries concurrently, making it significantly faster than traditional `ls` for directories with many files. The performance improvement is most noticeable with:

- Large directories (100+ files)
- Network-mounted filesystems
- Directories with mixed file types

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Code Coverage

The project includes comprehensive unit tests covering:

- Size formatting (human-readable and raw bytes)
- Time formatting and edge cases
- File type detection (hidden files, directories)
- Size conversion edge cases

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Dependencies

- **clap** (4.0+) - Command-line argument parsing
- **crossterm** (0.27) - Cross-platform terminal manipulation and colors
- **rayon** (1.7) - Data parallelism library
- **chrono** (0.4) - Date and time handling

## License

This project is open source and dedicated to the public domain under the [Unlicense](LICENSE).

## Acknowledgments

- Inspired by modern CLI tools like [`exa`](https://github.com/ogham/exa) and [`lsd`](https://github.com/lsd-rs/lsd)
- Built with the Rust ecosystem's excellent crates
- Optimized for developer workflow and terminal aesthetics
