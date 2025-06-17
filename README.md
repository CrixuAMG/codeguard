# CodeGuard

A Terminal User Interface (TUI) tool for Unix systems that helps developers maintain code quality by providing a unified interface for code style checks, analysis, linting, and testing.

## Features

- **Unified Interface**: Access all your code quality tools from one place
- **Keyboard Navigation**: Full keyboard support for efficient operation
- **Multi-language Support**: Works with Python, Node.js, Rust, and PHP projects
- **Automatic Tool Detection**: Automatically detects available tools in your project
- **Real-time Results**: See tool output in real-time
- **Configurable**: Customize tool settings and UI preferences

## Installation

```bash
# Using cargo
cargo install codeguard

# Or build from source
git clone https://github.com/crixuamg/codeguard.git
cd codeguard
cargo build --release
```

## Usage

```bash
# Run in the current directory
codeguard

# Run in a specific directory
codeguard /path/to/project
```

### Key Bindings

- `↑/↓` - Navigate tools
- `r` - Run selected tool
- `a` - Run all tools
- `?` - Show/hide help
- `q` - Quit

## Configuration

Configuration is stored in `~/.config/codeguard/config.yaml`. Example configuration:

```yaml
tools:
  python:
    flake8:
      enabled: true
      args: ["--max-line-length=100"]
    black:
      enabled: true
      args: ["--line-length=100"]
  javascript:
    eslint:
      enabled: true
      args: ["--fix"]
    prettier:
      enabled: true
      args: ["--write"]

ui:
  theme: "dark"
  show_tool_descriptions: true
```

## Supported Tools

### Python
- Flake8
- Black
- Pylint
- MyPy
- Pytest

### Node.js
- ESLint
- Prettier
- Jest
- TypeScript

### Rust
- Clippy
- Rustfmt
- Cargo test

### PHP
- PHP_CodeSniffer
- PHPStan
- PHPUnit

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

- GitHub: [@crixuamg](https://github.com/crixuamg) 