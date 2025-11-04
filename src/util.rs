use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 收集输入路径中的所有 .ncm 文件（支持单文件或目录）
pub fn collect_ncm_files(input: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if input.is_file() {
        if input.extension().map(|x| x == "ncm").unwrap_or(false) {
            files.push(input.to_path_buf());
        }
    } else if input.is_dir() {
        for entry in WalkDir::new(input)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            if entry
                .path()
                .extension()
                .map(|x| x == "ncm")
                .unwrap_or(false)
            {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files
}
