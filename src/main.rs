mod args;
mod decrypt;
mod util;

use args::CliArgs;
use decrypt::dump;
use util::collect_ncm_files;

use clap::Parser;
use rayon::prelude::*;
use std::path::Path;

fn main() {
    let args = CliArgs::parse();

    let input_path = Path::new(&args.input);
    let output_dir = args
        .output
        .as_ref()
        .map(|s| Path::new(s))
        .unwrap_or(Path::new("."));

    let files = collect_ncm_files(input_path);
    if files.is_empty() {
        eprintln!("❌ 未找到任何 .ncm 文件");
        return;
    }

    println!("🔍 共发现 {} 个 NCM 文件，开始解密...\n", files.len());

    files.par_iter().for_each(|file| {
        if let Err(e) = dump(file, output_dir) {
            eprintln!("⚠️ [{}] 解密失败: {}", file.display(), e);
        }
    });

    println!("\n🎵 全部任务完成！");
}
