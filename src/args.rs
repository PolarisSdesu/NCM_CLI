use clap::Parser;

/// 简单的 NCM 文件解密工具
///
/// 用法：
///   ncm ./song.ncm
///   ncm ./song.ncm ./export
///   ncm ./download
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// 输入文件或目录（支持通配符）
    pub input: String,

    /// 输出目录（可选，默认当前目录）
    pub output: Option<String>,
}
