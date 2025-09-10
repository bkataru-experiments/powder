# powder

`pwd` but better - A sophisticated replica of the `pwd` command in Rust with pretty printing and multicolor output.

## Features

- **Pretty printing**: Displays the current working directory with beautiful colors and formatting
- **Multiple command names**: Available as both `powder` (long name) and `pd` (short name)
- **Sophisticated CLI**: Full argument parsing with multiple options
- **Git integration**: Shows git repository information when available
- **Symlink handling**: Support for both logical (`-L`) and physical (`-P`) path resolution
- **Customizable output**: Custom separators, verbose mode, and color control
- **Pure Rust**: Built with minimal dependencies for maximum performance

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
cp target/release/powder /usr/local/bin/
cp target/release/pd /usr/local/bin/
```

## Usage

Basic usage:
```bash
powder          # Show current directory with colors
pd              # Same as above, short command
```

Options:
```bash
powder -v       # Verbose mode with detailed directory information
powder -g       # Show git repository information
powder -L       # Use logical path (follow PWD environment variable)
powder -P       # Use physical path (resolve all symlinks)
powder -s " → " # Use custom separator
powder --no-color # Disable colored output
```

Combined options:
```bash
powder -vg      # Verbose mode with git information
pd -Lg          # Logical path with git info
```

## Examples

### Basic usage
```bash
$ powder
home/runner/work/powder/powder
```

### Verbose mode
```bash
$ powder -v
Directory Information:
  Path: /home/runner/work/powder/powder
  Parent: /home/runner/work/powder
  Current Folder: powder
  Type: Directory
  Permissions: 755

home/runner/work/powder/powder
```

### Git repository information
```bash
$ powder -g
Git Repository Information:
  Repository Root: /home/runner/work/powder/powder
  Current Branch: main
  Status: Clean working tree

home/runner/work/powder/powder
```

### Custom separator
```bash
$ powder --separator " → "
home → runner → work → powder → powder
```

## Color Scheme

- **Root path**: Bright blue
- **Path components**: Bright white
- **Current directory**: Bright yellow (bold)
- **Separators**: Bright black
- **Git branch**: Bright yellow
- **Git status**: Green (clean) / Red (dirty)

## Dependencies

- `clap`: Command line argument parsing
- `colored`: Terminal color output

## License

MIT License - see LICENSE file for details.
