use crate::{FileError, Result};
use std::path::{Path, PathBuf};

/// 安全验证路径，防止路径穿越攻击
pub fn safe_resolve(base: &Path, path: &str) -> Result<PathBuf> {
    let resolved = base.join(path);
    
    // 检查是否包含 ".."
    if path.contains("..") {
        return Err(FileError::PathTraversalAttempt(resolved.clone()));
    }
    
    // 检查绝对路径
    if path.starts_with('/') || path.starts_with('~') {
        return Err(FileError::InvalidPath(path.to_string()));
    }
    
    // 检查路径长度
    if path.len() > 1024 {
        return Err(FileError::InvalidPath("路径过长".to_string()));
    }
    
    // 先检查基础目录是否存在
    if !base.exists() {
        return Err(FileError::NotFound(base.to_string_lossy().into_owned()));
    }
    
    let resolved_path = base.join(path);
    if !resolved_path.exists() {
        return Err(FileError::NotFound(path.to_string()));
    }
    
    // 使用 std::fs::read_dir 遍历路径组件验证
    let path_str = path.to_string();
    let parts: Vec<&str> = path_str.split('/').collect();
    
    let mut current = base.to_path_buf();
    for part in parts {
        if part == ".." {
            return Err(FileError::PathTraversalAttempt(resolved.clone()));
        }
        current.push(part);
    }
    
    // 最终验证路径在 base 下
    if !current.starts_with(base) {
        return Err(FileError::PathTraversalAttempt(current));
    }
    
    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_path_normal() {
        let base = Path::new("/tmp/test");
        std::fs::create_dir_all(base).unwrap();
        
        // 创建测试文件
        std::fs::write(base.join("file.txt"), "test").unwrap();
        
        let result = safe_resolve(base, "file.txt");
        assert!(result.is_ok(), "{:?}", result);
    }

    #[test]
    fn test_safe_path_traversal() {
        let base = Path::new("/tmp/test");
        
        // 尝试穿越到 /etc
        let result = safe_resolve(base, "../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_path_absolute() {
        let base = Path::new("/tmp/test");
        
        // 尝试使用绝对路径
        let result = safe_resolve(base, "/etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_path_long() {
        let base = Path::new("/tmp/test");
        
        // 尝试超长路径
        let long_path = "a".repeat(2048);
        let result = safe_resolve(base, &long_path);
        assert!(result.is_err());
    }
}
