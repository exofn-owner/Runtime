# 🚀 Runtime - Modern Uptime Utility

> A modern, colorful, and interactive replacement for the classic `uptime` command written in Rust.

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.1-green.svg)](Cargo.toml)

## ✨ Features

- 🎯 **Pin-point accuracy** - Direct `/proc` filesystem access for precise metrics
- 🌈 **Colorful output** - Beautiful color-coded displays with nerd fonts
- 📊 **Interactive tables** - Rich dashboard-style information presentation
- ⚡ **Fast performance** - Low-level system calls for minimal overhead
- 🐳 **Container support** - Special handling for containerized environments
- 🎪 **Multiple formats** - Standard, pretty, raw, since, and interactive modes
- ✨ **Loading animations** - Engaging visual feedback during data collection

## 🚀 Installation

### From Source

```bash
git clone https://github.com/Zer0C0d3r/runtime.git
cd runtime
cargo build --release
sudo cp target/release/runtime /usr/local/bin/
```

### Using Cargo

```bash
cargo install --path .
```

## 📖 Usage

### Basic Usage

```bash
# Interactive colorful dashboard (default)
runtime

# Standard uptime format
runtime --standard

# Pretty human-readable format
runtime --pretty

# Raw machine-readable values
runtime --raw

# Show boot time
runtime --since

# Container mode
runtime --container
```

### Output Examples

#### 🎨 Interactive Mode (Default)
```
Loading system metrics Done!

+=======================================================+
| *  SYSTEM UPTIME DASHBOARD  * |
+=======================================================+
| Current Time    : 19:36:59 +06:00               |
| System Uptime   : 27m 10s                        |
| Boot Time       : 2025-08-09 19:09:49        |
| Active Users    : 1 user                      |
| Load Average    : 1.26, 1.66, 1.39               |
| System Mode     : [NATIVE]               |
+=======================================================+
```

#### 📋 Standard Mode
```
 17:45:32 up 2:28, 3 users, load average: 1.24, 1.56, 1.89
```

#### 🎪 Pretty Mode
```
up 2 days, 14 hours, 28 minutes
```

#### 📊 Raw Mode
```
1754653037 219695.420000 876543.21 1.24 1.56 1.89
```

#### 📅 Since Mode
```
2025-08-06 03:17:17
```

## 🎛️ Command Line Options

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-i` | `--interactive` | Show interactive colorful table format (default) |
| `-s` | `--standard` | Show standard uptime format |
| `-p` | `--pretty` | Show uptime in pretty format |
| `-r` | `--raw` | Show uptime values in raw format |
| `-s` | `--since` | Show system boot time |
| `-c` | `--container` | Show container uptime indicators |
| `-V` | `--version` | Display version information |
| `-h` | `--help` | Show help message |

## 🏗️ Architecture

### Core Components

- **`system_metrics.rs`** - Low-level `/proc` filesystem access
- **`lib.rs`** - Core runtime logic and formatting
- **`cli.rs`** - Command line argument parsing
- **`main.rs`** - Entry point with loading animations

### System Metrics Collection

The utility reads directly from Linux `/proc` filesystem for maximum accuracy:

- `/proc/uptime` - System uptime and idle time
- `/proc/loadavg` - Load averages (1, 5, 15 minutes)
- `/proc/*/stat` - Process information for user counting
- `/proc/*/status` - Process ownership and terminal information

### User Counting Algorithm

Advanced multi-method user detection:

1. **Primary Method**: Scan `/proc/*/stat` for processes with controlling terminals
2. **Secondary Method**: Check `/proc/*/fd/*` for terminal file descriptors
3. **Fallback Method**: Environment-based detection for edge cases

## 🧪 Testing

Run the test suite:

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# With verbose output
cargo test -- --nocapture
```

### Test Coverage

- ✅ System metrics refresh accuracy
- ✅ Load average validation
- ✅ Boot time calculation
- ✅ User counting algorithms
- ✅ Format consistency

## 📊 Performance

Benchmarked against standard `uptime`:

| Metric | Runtime | Standard uptime |
|--------|---------|-----------------|
| Execution Time | ~2ms | ~1ms |
| Memory Usage | ~1.2MB | ~0.8MB |
| Accuracy | 100% | 100% |
| Features | 🌈 Rich | 📝 Basic |

## 🎨 Color Coding

Load averages are color-coded for quick system health assessment:

- 🟢 **Green** (< 1.0): Light load
- 🟡 **Yellow** (1.0-2.0): Moderate load
- 🟠 **Orange** (2.0-4.0): Heavy load
- 🔴 **Red** (> 4.0): Critical load

## 🐳 Container Support

When running in containers, the tool detects and displays:
- Container runtime indicators
- Adjusted user counting for containerized environments
- Special formatting for container-specific metrics

## 🔧 Configuration

No configuration files needed! All options available via command line flags.

### Environment Variables

- `DISPLAY` / `WAYLAND_DISPLAY` - Used for user detection fallback
- `UID` - Current user identification

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash
git clone https://github.com/Zer0C0d3r/runtime.git
cd runtime
cargo build
cargo test
./target/debug/runtime --interactive
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Original `uptime` utility from procps-ng
- Rust community for excellent crates
- Linux kernel developers for `/proc` filesystem

## 🐛 Bug Reports

Found a bug? Please open an issue with:
- Your OS and kernel version
- Rust version (`rustc --version`)
- Command that caused the issue
- Expected vs actual output

## 📈 Roadmap

- [ ] 🏃 Real-time monitoring mode
- [ ] 📱 JSON output format
- [ ] 🔌 Plugin system
- [ ] 📊 Historical data tracking
- [ ] 🌐 Network metrics integration
- [ ] 🎮 Interactive TUI mode

---

Made with ❤️ in Rust 🦀
