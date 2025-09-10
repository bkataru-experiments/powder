# powder

`pwd` but better - A sophisticated replica of the `pwd` command in Rust with pretty printing, themes, history tracking, and responsive output.

## Features

- **🎨 Beautiful Themes**: 5 built-in color themes (default, ocean, forest, sunset, mono) with easy switching
- **📚 Directory History**: Automatic tracking of your last 10 working directories with timestamps
- **📱 Responsive Design**: Smart output adaptation for different terminal sizes with compact mode
- **🔧 Comprehensive CLI**: Full argument parsing with extensive options and help
- **🚀 Performance Optimized**: Fast path resolution with minimal system calls
- **🎯 Git Integration**: Shows git repository information when available
- **🔗 Symlink Handling**: Support for both logical (`-L`) and physical (`-P`) path resolution
- **🎛️ Customizable Output**: Custom separators, verbose mode, and color control
- **⚡ Pure Rust**: Built with minimal dependencies for maximum performance
- **🧪 Thoroughly Tested**: Comprehensive test suite with 20+ integration tests

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

### Basic Usage
```bash
powder          # Show current directory with colors
pd              # Same as above, short command
```

### Core Options
```bash
powder -v       # Verbose mode with detailed directory information
powder -g       # Show git repository information
powder -L       # Use logical path (follow PWD environment variable)
powder -P       # Use physical path (resolve all symlinks)
powder -s " → " # Use custom separator
powder --no-color # Disable colored output
```

### Theme System
```bash
powder --themes           # List all available themes
powder --theme ocean      # Switch to ocean theme
powder --theme forest     # Switch to forest theme
powder --theme sunset     # Switch to sunset theme
powder --theme mono       # Switch to monochrome theme
```

### History Tracking
```bash
powder --history          # Show your last 10 working directories
```

### Responsive Output
```bash
powder --compact          # Force compact mode for narrow terminals
```

### Combined Options
```bash
powder -vg --theme ocean # Verbose + git info with ocean theme
pd -Lg --compact         # Logical path + git info in compact mode
```

## Examples

### Basic Usage with Themes
```bash
$ powder
home/runner/work/powder/powder

$ powder --theme ocean
home/runner/work/powder/powder  # (displayed in blue-cyan colors)

$ powder --theme sunset  
home/runner/work/powder/powder  # (displayed in warm red-yellow colors)
```

### Verbose Mode
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

### Git Repository Information
```bash
$ powder -g
Git Repository Information:
  Repository Root: /home/runner/work/powder/powder
  Current Branch: main
  Status: Clean working tree

home/runner/work/powder/powder
```

### Directory History
```bash
$ powder --history
Directory History:
  1: /home/runner/work/powder/powder (current) (just now)
  2: /tmp (2m ago)
  3: /home/runner (5m ago)
  4: /usr/local/bin (1h ago)
```

### Custom Separator
```bash
$ powder --separator " → "
home → runner → work → powder → powder
```

### Responsive Design
```bash
# On narrow terminals (or with --compact):
$ powder --compact
home/ru.../work/po.../powder

# Long paths get intelligently compressed:
$ powder --compact
tmp/very/deep/di…/st…/that/sh…/tr…/co…/mode
```

## Available Themes

| Theme | Description | Color Palette |
|-------|-------------|---------------|
| **default** | Classic powder colors | Bright blues, whites, yellows |
| **ocean** | Calming oceanic blues | Blues, cyans, greens |
| **forest** | Natural green palette | Greens, bright greens, yellows |
| **sunset** | Warm sunset colors | Reds, yellows, oranges |
| **mono** | Minimalist grayscale | Whites, grays, bright whites |

## Advanced Features

### Smart Path Compression
When terminal width is limited, powder automatically compresses long directory names while preserving readability:
- Always shows the current directory name in full
- Compresses middle components with ellipsis (`…`)
- Maintains path structure for navigation context

### History Persistence
Directory history is automatically saved to `~/.powder.history` (or equivalent config directory) and includes:
- Last 10 unique directories visited
- Automatic deduplication (visiting same dir moves it to top)
- Timestamps with human-readable relative times
- JSON format for reliability and extensibility

### Performance Optimizations
- Lazy git repository detection with depth limiting
- Environment variable caching for logical paths
- Minimal file system operations
- Fast path validation and compression

## Configuration

Powder automatically detects your terminal capabilities and adjusts output accordingly:
- **Terminal width detection** for responsive path display
- **Color capability detection** respects `NO_COLOR` environment variable
- **Configuration directory selection** follows system conventions

## Dependencies

- `clap`: Command line argument parsing
- `colored`: Terminal color output  
- `serde` & `serde_json`: History file serialization
- `dirs`: System directory detection
- `terminal_size`: Terminal dimension detection

## Testing

Run the comprehensive test suite:
```bash
cargo test
```

The project includes:
- **Unit tests** for core functionality
- **Integration tests** for CLI behavior
- **Edge case testing** for error conditions
- **Performance regression testing**

## License

MIT License - see LICENSE file for details.
