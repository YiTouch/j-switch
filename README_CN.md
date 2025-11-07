# jsh - JDK 切换助手

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue.svg)](https://github.com/yourusername/j-switch)

[English](README.md) | [中文文档](README_CN.md)

## 中文文档

一个使用 Rust 编写的JDK 版本管理和切换命令行工具。

### ✨ 功能特性

- 🔍 **自动检测**：自动扫描并检测常见目录中的 JDK 安装
- 🔄 **快速切换**：一条命令即可切换不同的 JDK 版本
- ⚙️ **环境管理**：自动更新 `JAVA_HOME` 和 `PATH` 环境变量
- 📦 **版本管理**：列出、安装和管理多个 JDK 版本
- 💾 **持久化配置**：将 JDK 配置保存在 `config.json`
- 🎨 **美观界面**：彩色、直观的命令行界面

### 📋 目录

- [系统要求](#-系统要求)
- [重要提示](#️-重要提示)
- [安装方法](#-安装方法)
- [快速开始](#-快速开始)
- [命令说明](#-命令说明)
- [使用示例](#-使用示例)
- [配置文件](#️-配置文件)
- [故障排除](#-故障排除)
- [贡献指南](#-贡献指南)
- [开源协议](#-开源协议)
- [致谢](#-致谢)
- [开发路线图](#️-开发路线图)

### 🔧 系统要求

- **操作系统**：Windows 10+
- **Rust**：1.70 或更高版本（仅编译时需要）
- **JDK**：至少一个 JDK 安装（版本 7+）

### ⚠️ 重要提示

> **关于 JDK 下载功能**  
> 本工具使用的是 GitHub 上的 OpenJDK 资源。由于中国国内网络环境限制（GFW），**下载 JDK 功能需要科学上网**才能正常使用。  
> 除下载功能外，其他所有功能（如列出、切换、管理已安装的 JDK）均不受网络影响，可正常使用。

> **JDK路径扫描深度**  
> jsh扫描的路径深度为 5 层，避免目录过深扫描时间过长，所以请将JDK安装在浅层目录下。  

> **JDK 下载源说明**  
> 本工具的 JDK 下载功能使用以下官方源：
> - **主要来源**：[Adoptium (Eclipse Temurin)](https://adoptium.net/) - 提供经过 TCK 认证的高质量 OpenJDK 二进制文件
> - **API 接口**：通过 Adoptium 的官方 API 获取可用版本列表和下载链接
> - **资源托管**：下载资源托管在 GitHub，确保稳定性和可靠性
>
> 所有下载的 JDK 均为官方认证的 OpenJDK 发行版，安全可靠。

### 📦 安装方法

#### 方法 1：下载预编译二进制文件

从 [Releases](https://github.com/YiTouch/j-switch/releases) 页面下载最新版本。

#### 方法 2：从源码编译

```bash
# 克隆仓库
git clone https://github.com/YiTouch/j-switch.git
cd j-switch

# 编译 Release 版本
cargo build --release

# 二进制文件位于 target/release/jsh.exe（Windows）
```

### 🚀 快速开始

1. **列出所有检测到的 JDK**：
```bash
jsh list
```

2. **切换到指定 JDK 版本**：
```bash
jsh use 17
```

3. **查看当前激活的 JDK**：
```bash
jsh current
```

### 📝 命令说明

| 命令 | 说明 | 示例 |
|------|------|------|
| `jsh list` | 列出所有检测到的 JDK | `jsh list` |
| `jsh current` | 显示当前激活的 JDK | `jsh current` |
| `jsh use <版本>` | 切换到指定 JDK 版本 | `jsh use 17` |
| `jsh download <版本>` | 下载并安装 JDK（即将推出） | `jsh download 21` |
| `jsh search [版本]` | 搜索可用的 JDK 版本（即将推出） | `jsh search 17` |
| `jsh --help` | 显示帮助信息 | `jsh --help` |

### 💡 使用示例

#### 列出 JDK 安装

```bash
$ jsh list

正在扫描 JDK 安装...

已安装的 JDK：
================================================================================
* JDK 17 (当前)
  版本: 17.0.10
  供应商: OpenJDK
  路径: C:\Program Files\Java\jdk-17

- JDK 11
  版本: 11.0.8
  路径: C:\Program Files\Java\jdk-11.0.8

- JDK 8
  版本: 1.8.0_291
  供应商: Oracle
  路径: C:\Program Files\Java\jdk1.8.0_291

--------------------------------------------------------------------------------
总计: 3 个 JDK
```

#### 切换 JDK 版本

```bash
$ jsh use 11

已更新配置:
  版本: JDK 11
  路径: C:\Program Files\Java\jdk-11.0.8

正在更新环境变量...
[OK] 已设置 JAVA_HOME 为: C:\Program Files\Java\jdk-11.0.8
[OK] 已更新 PATH，包含: C:\Program Files\Java\jdk-11.0.8\bin

[OK] 成功切换到 JDK

注意:
  环境变量已更新。
  您可能需要重启终端以使更改生效。
```

#### 查看当前 JDK

```bash
$ jsh current

当前 JDK:
============================================================
版本: JDK 11
完整版本: 11.0.8
路径: C:\Program Files\Java\jdk-11.0.8
============================================================

[OK] JAVA_HOME 已正确设置
```

### ⚙️ 配置文件

jsh 将配置存储在 `config.json`：

配置示例：
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

#### 添加到 PATH

**Windows**：
```powershell
# 临时添加到 PATH（当前会话）
$env:Path += ";D:\xx\xx\(j-switch.exe)"

# 永久添加到 PATH（以管理员身份运行 PowerShell）
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";D:\xx\xx\(j-switch.exe)", "User")
```

### 🐛 故障排除

#### JDK 未被检测到

如果您的 JDK 未被自动检测：

1. **检查JDK路径深度**：
   - jsh 只会扫描深度为 5 的目录，避免目录过深扫描时间过长！！


#### 环境变量未更新（Windows）

1. **切换 JDK 后重启终端**
2. **检查是否以管理员权限运行**（系统级更改需要）
3. **手动验证**：`echo %JAVA_HOME%`

### 🤝 贡献指南

欢迎贡献！

### 📄 开源协议

本项目采用 MIT 协议 - 详见 [LICENSE](LICENSE) 文件。

### 🙏 致谢

- 灵感来自nvm工具


### 🗺️ 开发路线图

- [x] JDK 自动检测
- [x] JDK 版本切换
- [x] 环境变量管理
- [x] JDK 下载和安装
- [x] 版本搜索功能

---

用 ❤️ 和 🦀 Rust 制作
