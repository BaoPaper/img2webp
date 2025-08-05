# i2w

一个高效的命令行工具，用于将图片文件转换为 WebP 格式。支持单文件转换、批量文件夹转换、并发处理、进度显示等功能，使用 FFmpeg 作为后端处理工具。

## 功能特性

- 🖼️ **多种格式支持**：支持 JPG, PNG, BMP, TIFF 等常见图片格式转换为 WebP
- 📁 **批量处理**：支持单文件和整个文件夹的图片转换
- 🔁 **递归扫描**：可递归处理子文件夹中的图片
- ⚙️ **质量控制**：可自定义转换质量 (0-100)
- ⚡ **并发处理**：支持多线程并发转换，提高处理速度
- 📊 **进度显示**：实时显示转换进度和状态
- 🔄 **原文件处理**：可选择是否替换原文件
- 🛡️ **错误处理**：完善的错误处理和用户友好的提示
- 🖥️ **跨平台**：支持 Windows, Linux, macOS

## 安装要求

在使用此工具之前，请确保系统已安装以下软件：

1. **Rust**：需要 Rust 1.60 或更高版本
2. **FFmpeg**：需要 FFmpeg 并确保其在系统 PATH 中

### 安装 FFmpeg

#### Windows
从 [FFmpeg官网](https://ffmpeg.org/download.html) 下载并安装，确保将 FFmpeg 添加到系统 PATH。

#### macOS
```bash
# 使用 Homebrew
brew install ffmpeg
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install ffmpeg
```

## 安装

通过 Cargo 安装：

```bash
cargo install i2w
```

确保您的 `~/.cargo/bin` 目录在您的 `PATH` 环境变量中。

## 使用方法

### 基本语法
```bash
i2w [OPTIONS] <INPUT>
```

### 参数说明

```
ARGS:
    <INPUT>     输入文件或文件夹路径

OPTIONS:
    -h, --help              打印帮助信息
    -o, --output <OUTPUT>   输出文件或文件夹路径
    -q, --quality <QUALITY> 转换质量 [default: 80]
    -r, --recursive         递归处理子文件夹
    -c, --concurrent <N>    并发处理数 [default: 4]
    --replace               替换原文件
    -V, --version           打印版本信息
```

### 使用示例

```bash
# 转换单个文件
i2w image.jpg

# 转换单个文件并指定输出路径
i2w image.png -o output.webp

# 转换单个文件并指定质量
i2w -q 90 image.png

# 转换文件夹中的所有图片
i2w /path/to/images

# 转换文件夹并递归处理子文件夹
i2w -r /path/to/images

# 转换并替换原文件
i2w --replace /path/to/images

# 设置并发数为8
i2w -c 8 /path/to/images

# 组合多个选项
i2w -r -c 6 -q 85 /path/to/images
```

## 项目结构

```
src/
├── main.rs          # 程序入口和主逻辑
├── cli.rs           # 命令行参数解析
├── converter.rs     # 转换核心逻辑
├── scanner.rs       # 文件扫描和识别
├── progress.rs      # 进度显示
└── error.rs         # 错误处理
```

## 技术架构

本工具采用模块化设计，主要包含以下模块：

1. **命令行接口模块**：使用 clap 库解析命令行参数
2. **文件扫描模块**：识别和扫描图片文件
3. **转换核心模块**：调用 FFmpeg 进行图片转换
4. **进度显示模块**：使用 indicatif 库显示进度条
5. **错误处理模块**：统一错误处理和提示
6. **主程序模块**：协调各模块工作

## 性能优化

- 使用 tokio 异步运行时实现并发处理
- 通过并发控制参数平衡性能和系统资源占用
- 实时进度显示让用户了解转换状态

## 错误处理

工具具有完善的错误处理机制，能够处理以下类型的错误：

- 参数错误
- 文件不存在或无权限
- FFmpeg 执行失败
- 系统资源不足

所有错误都会以用户友好的方式显示。

## 许可证

本项目采用 [MIT License](LICENSE) 开源许可证。

## 贡献

欢迎提交 Issue 和 Pull Request 来改进此工具。