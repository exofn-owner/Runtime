https://github.com/exofn-owner/Runtime/releases

# Runtime — Modern Rust uptime tool for Unix, macOS, Termux 🚀🖥️

[![Releases](https://img.shields.io/badge/-Releases-blue?style=for-the-badge&logo=github)](https://github.com/exofn-owner/Runtime/releases) [![Rust](https://img.shields.io/badge/Rust-1.72+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-green?style=flat-square)](#license)  
![terminal](https://raw.githubusercontent.com/github/explore/main/topics/terminal/terminal.png) ![rust-logo](https://raw.githubusercontent.com/github/explore/main/topics/rust/rust.png)

A sleek, colorful uptime alternative built in Rust. Runtime shows system uptime, load, and session info in a compact, readable format. It targets Unix-like systems: Linux, macOS, OpenBSD, Termux. It runs in a shell and plays well with dotfiles, status bars, and CI.

Badges: cli • command-line-tool • linux • macos • modern-alternative • openbsd • runtime • rust • shell • termux • unix • uptime

Quick links
- Releases and prebuilt binaries: https://github.com/exofn-owner/Runtime/releases
- Source: this repository

Download and run (Releases)
- The releases page contains prebuilt assets. Download the binary or tarball for your OS and execute it.
- Example steps for a Linux x86_64 release asset:
  - curl -L -o runtime-linux.tar.gz "https://github.com/exofn-owner/Runtime/releases/download/vX.Y.0/runtime-linux-x86_64.tar.gz"
  - tar -xzf runtime-linux.tar.gz
  - chmod +x runtime
  - sudo mv runtime /usr/local/bin/runtime
  - runtime --help

If you prefer, visit the releases page in a browser: https://github.com/exofn-owner/Runtime/releases. The release pages list assets, checksums, and SHA signatures.

Why Runtime
- Clear output. Runtime presents uptime data in a compact row. It uses color to highlight key metrics.
- Small binary. The Rust static binary stays small and runs fast.
- Single-file install. Drop the binary into /usr/local/bin or use a package manager.
- Shell first. Runtime outputs plain text and supports ANSI color, JSON, and machine mode.
- Cross-platform. Builds target Linux, macOS, OpenBSD, and Termux.

Features
- Uptime, idle, and boot time.
- Load averages split by cores.
- Active user sessions and terminal counts.
- Terminal-friendly colors and icons.
- JSON output for scripts.
- Shell completions (bash, zsh, fish).
- Exit codes that reflect system state.
- Minimal runtime dependencies.

Quick start examples

Show a compact summary:
```
runtime
```

Show verbose output with colors:
```
runtime --verbose --color
```

Get JSON for scripts:
```
runtime --json
```

Install from Releases (example)
- Pick the correct release asset for your platform on https://github.com/exofn-owner/Runtime/releases.
- Linux example:
  ```
  curl -L -o runtime-linux.tar.gz "https://github.com/exofn-owner/Runtime/releases/download/vX.Y.0/runtime-linux-x86_64.tar.gz"
  tar -xzf runtime-linux.tar.gz
  chmod +x runtime
  sudo mv runtime /usr/local/bin/
  ```
- macOS example:
  ```
  curl -L -o runtime-macos.tar.gz "https://github.com/exofn-owner/Runtime/releases/download/vX.Y.0/runtime-macos-x86_64.tar.gz"
  tar -xzf runtime-macos.tar.gz
  chmod +x runtime
  sudo mv runtime /usr/local/bin/
  ```
- Termux / Android:
  - Download the arm or aarch64 asset from the releases page.
  - Make the file executable and move it into $PREFIX/bin.

Build from source
- Install Rust and cargo.
- Clone the repo and build:
  ```
  git clone https://github.com/exofn-owner/Runtime.git
  cd Runtime
  cargo build --release
  sudo cp target/release/runtime /usr/local/bin/runtime
  ```

Package manager options
- If a distro package exists, use your package manager.
- Otherwise, use the release binary or build with cargo.

Usage and flags
- runtime [OPTIONS]
- Common options:
  - --help            Show help.
  - --version         Show version.
  - --json            Output JSON.
  - --no-color        Disable ANSI colors.
  - --color <when>    Control colors: always, auto, never.
  - --verbose         Show details: sessions, users, boot logs.
  - --machine         Minimal output for scripts.
  - --interval <sec>  Update interval for watch mode.
  - --watch           Refresh the display every N seconds.

Examples
- Watch uptime every 5s:
  ```
  runtime --watch --interval 5
  ```
- Show machine-readable fields:
  ```
  runtime --machine
  ```
- Pipe JSON into jq:
  ```
  runtime --json | jq '.load.average'
  ```

Output format examples
- Plain mode:
  ```
  ⏱  up 10:42, 3 users, load: 0.22 0.18 0.15
  ```
- JSON:
  ```
  {
    "uptime": "10:42",
    "users": 3,
    "load": [0.22, 0.18, 0.15],
    "boot_time": "2025-08-18T06:30:00Z"
  }
  ```

Exit codes
- 0 OK
- 1 Generic error
- 2 Invalid arguments
- 3 Platform not supported

Integrations
- Status bars: print runtime in your i3bar, swaybar, or tmux status line.
- Dotfiles: add runtime to your shell prompt or MOTD.
- System monitoring: use JSON mode to feed runtime into scripts and alerting.

Shell completion
- Generate completion scripts:
  - Bash:
    ```
    runtime completions bash > /etc/bash_completion.d/runtime
    ```
  - Zsh:
    ```
    runtime completions zsh > ~/.zfunc/_runtime
    ```
  - Fish:
    ```
    runtime completions fish > ~/.config/fish/completions/runtime.fish
    ```

Systemd service example
- Use a simple systemd unit to run runtime in a monitoring session. Replace ExecStart with the path to the binary.
  ```
  [Unit]
  Description=Runtime uptime poller

  [Service]
  ExecStart=/usr/local/bin/runtime --watch --interval 60
  Restart=on-failure

  [Install]
  WantedBy=multi-user.target
  ```

Security and checksums
- Releases include checksums and signatures. Verify the downloaded asset before running. The releases page lists SHA256 sums next to assets.

Config file (optional)
- Runtime supports a minimal TOML config at ~/.config/runtime/config.toml:
  ```
  [display]
  color = "auto"
  icons = true

  [output]
  mode = "compact" # compact | verbose | json
  ```
- The binary reads config, then merges CLI flags.

Screenshots and visuals
- Terminal output example:
  ![screenshot-compact](https://raw.githubusercontent.com/exofn-owner/Runtime/main/assets/screenshot-compact.png)
- JSON mode:
  ![screenshot-json](https://raw.githubusercontent.com/exofn-owner/Runtime/main/assets/screenshot-json.png)

Contributing
- Open issues for bugs or feature requests.
- Fork the repo and send a pull request for code changes.
- Follow the repo style: short commits, clear messages, small changes.
- Run tests before submitting:
  ```
  cargo test
  cargo fmt -- --check
  cargo clippy -- -D warnings
  ```

Testing and CI
- The repo runs CI on push. CI builds for Linux and macOS and runs unit tests.
- Use the checks in CI to validate cross-platform builds.

Design notes
- Runtime uses native OS calls where possible for accurate uptime and session data.
- It avoids heavy dependencies and focuses on single-task reliability.
- The color palette uses readable contrasts for terminals.

Troubleshooting
- If the binary fails to run, verify execute permission:
  ```
  chmod +x /usr/local/bin/runtime
  ```
- If a feature seems missing on your platform, check issue tracker.
- If a release asset does not match your architecture, build from source.

License
- Dual license MIT OR Apache-2.0. See LICENSE files for details.

Acknowledgements and links
- Built with Rust and system APIs.
- Releases: https://github.com/exofn-owner/Runtime/releases
- Rust: https://www.rust-lang.org/
- Terminal icons: GitHub Explore topics

Maintainers
- Main repo owner: exofn-owner
- Community contributions welcome via pull requests and issues.

Contact
- Open an issue on the repo for questions, feature requests, or bug reports.