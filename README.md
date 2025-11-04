# 🎵 NCM CLI

**NCM CLI** 是一个由 Rust 构建的轻量级的命令行工具，用于**解密并转换网易云音乐 `.ncm` 文件**为标准音频格式（如 `.mp3`、`.flac` 等）。

---

## 安装 / Installation / インストール

### 📥 使用安装程序（推荐）
下载并运行：
```
ncm_setup.exe
```

安装程序会：
- 将 `ncm.exe` 安装到系统目录（默认：`C:\Program Files\ncm\`）；
- 将该目录自动添加到系统环境变量 `PATH`；

安装完成后，打开命令提示符或 PowerShell，输入：
```bash
ncm --help
````

如果命令可用，则安装成功。

---

## ⚙️ 使用 / Usage / 使い方

### 基本命令

```bash
ncm <file_or_directory>
```

### 示例

```bash
ncm D:\Music\song.ncm
```

将 `.ncm` 文件解密为 `.mp3` 或 `.flac` 文件，输出到当前目录。

---

## ⚙️ 参数说明 / Options / オプション

| 参数            | 说明                                                                      | English                                          | 日本語                                           |
| --------------- | ------------------------------------------------------------------------- | ------------------------------------------------ | ------------------------------------------------ |
| `<INPUT>`       | 输入文件或目录（支持通配符）<br>示例：`ncm ./song.ncm` 或 `ncm ./music/*` | Input file or directory (supports wildcards)     | 入力ファイルまたはフォルダ（ワイルドカード対応） |
| `[OUTPUT]`      | 输出目录（可选，默认当前目录）<br>示例：`ncm ./song.ncm ./export`         | Optional output directory (default: current dir) | 出力先（省略可、既定は現在のフォルダ）           |
| `-h, --help`    | 显示帮助信息                                                              | Show help                                        | ヘルプを表示                                     |
| `-V, --version` | 显示版本号                                                                | Show version                                     | バージョン情報を表示                             |


## 📁 示例 / Examples / 例

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
ncm D:\Download\ -r -o D:\Export\
```

---

## 提示 / Tips / ヒント

* 输出文件会自动保持原始文件名，只更改扩展名。
  例如：

  ```
  input:  demo.ncm
  output: demo.mp3
  ```
* 工具会自动检测 `.ncm` 文件类型（MP3 / FLAC），并支持多线程并行解密。

---

## 注意事项 / Notes / 注意

* 本工具仅供个人学习与备份使用。
  请勿将解密后的音频文件用于任何商业用途。
* 建议在命令行中使用，不支持图形界面。

---

## 开发信息 / Developer Info / 開発者情報

**NCM CLI**  
Developed with ❤️ in Rust  
© 2025 PolarisS夜搠星