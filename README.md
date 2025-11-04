# NCM CLI

**NCM CLI** 是一个由 Rust 构建的轻量级命令行工具，用于 **解密并转换网易云音乐 `.ncm` 文件** 为标准音频格式（如 `.mp3`、`.flac` 等）。

---

## 安装

### 使用安装程序
下载并运行：
```
ncm_setup.exe
```

安装程序会：
- 将 `ncm.exe` 安装到系统目录（默认：`C:\Program Files\ncm\`）；
- 自动将该目录添加到系统环境变量 `PATH`；

安装完成后，打开命令提示符或 PowerShell，输入：
```bash
ncm --help
```

如果命令可用，则表示安装成功。

---

## 使用方法

### 基本命令

```bash
ncm <文件或目录>
```

### 示例

```bash
ncm D:\Music\song.ncm
```

该命令会将 `.ncm` 文件解密为 `.mp3` 或 `.flac` 文件，并输出到当前目录。

---

## 参数说明

| 参数              | 说明              | 示例                                 |
| --------------- | --------------- | ---------------------------------- |
| `<INPUT>`       | 输入文件或目录（支持通配符）  | `ncm ./song.ncm` 或 `ncm ./music/*` |
| `[OUTPUT]`      | 输出目录（可选，默认当前目录） | `ncm ./song.ncm ./export`          |
| `-h, --help`    | 显示帮助信息          |                                    |
| `-V, --version` | 显示版本号           |                                    |

---

## 使用示例

### 解密单个文件并输出到当前目录

```bash
ncm D:\Music\demo.ncm
```

### 解密单个文件并输出到指定目录

```bash
ncm D:\Music\demo.ncm D:\Export\
```

### 批量处理整个文件夹并输出到当前目录

```bash
ncm D:\Download\
```

### 批量处理整个文件夹并输出到指定目录

```bash
ncm D:\Download\ D:\Export\
```

---

**NCM CLI**
Developed with ❤️ in Rust
