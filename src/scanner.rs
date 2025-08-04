use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 检查文件是否为支持的图片格式
pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        match ext.to_str() {
            Some("jpg") | Some("jpeg") | Some("png") | Some("bmp") | Some("tiff") | Some("tif") => {
                return true;
            }
            _ => return false,
        }
    }
    false
}

/// 扫描目录中的图片文件
pub fn scan_image_files(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if recursive {
        // 递归扫描所有子目录
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && is_image_file(path) {
                files.push(path.to_path_buf());
            }
        }
    } else {
        // 只扫描当前目录
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && is_image_file(&path) {
                files.push(path);
            }
        }
    }
    
    Ok(files)
}