pub mod safe_path;
pub mod list;
pub mod search;

use std::path::{Path, PathBuf};
use std::io;

#[derive(Debug)]
pub enum FileError {
    PathTraversalAttempt(PathBuf),
    InvalidPath(String),
    NotFound(String),
    PermissionDenied,
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::PathTraversalAttempt(path) => 
                write!(f, "路径穿越尝试: {}", path.display()),
            FileError::InvalidPath(path) => write!(f, "无效路径: {}", path),
            FileError::NotFound(path) => write!(f, "文件不存在: {}", path),
            FileError::PermissionDenied => write!(f, "权限不足"),
        }
    }
}

impl std::error::Error for FileError {}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => FileError::NotFound(err.to_string()),
            io::ErrorKind::PermissionDenied => FileError::PermissionDenied,
            _ => FileError::InvalidPath(err.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, FileError>;

// 导出 safe_resolve
pub use crate::safe_path::safe_resolve;

#[cfg(test)]
mod tests {
    use crate::safe_resolve;
    use std::path::Path;

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
