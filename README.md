# jsh - JDK Switch Helper

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://github.com/YiTouch/j-switch)

[English](#english-documentation) | [中文文档](README_CN.md)

## English Documentation

A fast, cross-platform JDK version management and switching command-line tool written in Rust.

### ✨ Features

- 🔍 **Auto-Detection**: Automatically scans and detects JDK installations in common directories
- 🔄 **Quick Switching**: Switch between different JDK versions with a single command
- ⚙️ **Environment Management**: Automatically updates `JAVA_HOME` and `PATH` environment variables
- 📦 **Version Management**: List, install, and manage multiple JDK versions
- 💾 **Persistent Configuration**: Saves JDK configuration in `config.json`
- 🎨 **Beautiful Interface**: Colorful and intuitive command-line interface

### 📋 Table of Contents

- [System Requirements](#-system-requirements)
- [Important Notes](#️-important-notes)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Command Reference](#-command-reference)
- [Usage Examples](#-usage-examples)
- [Configuration File](#️-configuration-file)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)
- [Acknowledgments](#-acknowledgments)
- [Roadmap](#️-roadmap)

### 🔧 System Requirements

- **Operating System**: Windows 10+
- **Rust**: 1.70 or higher (only required for compilation)
- **JDK**: At least one JDK installation (version 7+)

### ⚠️ Important Notes

> **JDK Path Scanning Depth**  
> jsh scans directories up to 5 levels deep to avoid excessive scanning time. Please install JDKs in shallow directory structures.

> **JDK Download Source**  
> This tool uses the following official sources for JDK downloads:
> - **Primary Source**: [Adoptium (Eclipse Temurin)](https://adoptium.net/) - Provides high-quality, TCK-certified OpenJDK binaries
> - **API Interface**: Retrieves available version lists and download links through Adoptium's official API
> - **Resource Hosting**: Download resources are hosted on GitHub for stability and reliability
>
> All downloaded JDKs are officially certified OpenJDK distributions, ensuring security and reliability.

### 📦 Installation

#### Method 1: Download Pre-compiled Binary

Download the latest version from the [Releases](https://github.com/YiTouch/j-switch/releases) page.

#### Method 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/YiTouch/j-switch.git
cd j-switch

# Build release version
cargo build --release

# Binary located at target/release/jsh.exe (Windows)
```

### 🚀 Quick Start

1. **List all detected JDKs**:
```bash
jsh list
```

2. **Switch to a specific JDK version**:
```bash
jsh use 17
```

3. **View currently active JDK**:
```bash
jsh current
```

### 📝 Command Reference

| Command | Description | Example |
|---------|-------------|---------|
| `jsh list` | List all detected JDKs | `jsh list` |
| `jsh current` | Display currently active JDK | `jsh current` |
| `jsh use <version>` | Switch to specified JDK version | `jsh use 17` |
| `jsh download <version>` | Download and install JDK (Coming Soon) | `jsh download 21` |
| `jsh search [version]` | Search available JDK versions (Coming Soon) | `jsh search 17` |
| `jsh --help` | Display help information | `jsh --help` |

### 💡 Usage Examples

#### List JDK Installations

```bash
$ jsh list

Scanning for JDK installations...

Installed JDKs:
================================================================================
* JDK 17 (current)
  Version: 17.0.10
  Vendor: OpenJDK
  Path: C:\Program Files\Java\jdk-17

- JDK 11
  Version: 11.0.8
  Path: C:\Program Files\Java\jdk-11.0.8

- JDK 8
  Version: 1.8.0_291
  Vendor: Oracle
  Path: C:\Program Files\Java\jdk1.8.0_291

--------------------------------------------------------------------------------
Total: 3 JDKs
```

#### Switch JDK Version

```bash
$ jsh use 11

Configuration updated:
  Version: JDK 11
  Path: C:\Program Files\Java\jdk-11.0.8

Updating environment variables...
[OK] JAVA_HOME set to: C:\Program Files\Java\jdk-11.0.8
[OK] PATH updated to include: C:\Program Files\Java\jdk-11.0.8\bin

[OK] Successfully switched to JDK

Note:
  Environment variables have been updated.
  You may need to restart your terminal for changes to take effect.
```

#### View Current JDK

```bash
$ jsh current

Current JDK:
============================================================
Version: JDK 11
Full Version: 11.0.8
Path: C:\Program Files\Java\jdk-11.0.8
============================================================

[OK] JAVA_HOME is correctly set
```

### ⚙️ Configuration File

jsh stores configuration in `config.json`:

Configuration example:
```json
{
  "jdks": {
    "11": {
      "path": "C:\\Program Files\\Java\\jdk-11.0.8",
      "version": "11",
      "vendor": null,
      "java_version": "11.0.8"
    },
    "17": {
      "path": "C:\\Program Files\\Java\\jdk-17",
      "version": "17",
      "vendor": "OpenJDK",
      "java_version": "17.0.10"
    }
  },
  "current_jdk": "17"
}
```

#### Adding to PATH

**Windows**:
```powershell
# Temporarily add to PATH (current session)
$env:Path += ";D:\xx\xx\(jsh.exe)"

# Permanently add to PATH (run PowerShell as Administrator)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";D:\xx\xx\(jsh.exe)", "User")
```

### 🐛 Troubleshooting

#### JDK Not Detected

If your JDK is not automatically detected:

1. **Check JDK path depth**:
   - jsh only scans directories up to 5 levels deep to avoid excessive scanning time

#### Environment Variables Not Updated (Windows)

1. **Restart terminal after switching JDK**
2. **Check if running with administrator privileges** (required for system-level changes)
3. **Manually verify**: `echo %JAVA_HOME%`

### 🤝 Contributing

Contributions are welcome!

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### 🙏 Acknowledgments

- Inspired by the nvm tool

### 🗺️ Roadmap

- [x] Automatic JDK detection
- [x] JDK version switching
- [x] Environment variable management
- [x] JDK download and installation
- [x] Version search functionality

---

Made with ❤️ and 🦀 Rust
