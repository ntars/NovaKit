pub mod resize;
pub mod compress;

use std::io;

#[derive(Debug)]
pub enum ImageError {
    FileNotFound(String),
    InvalidFormat(String),
    PermissionDenied,
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::FileNotFound(path) => write!(f, "文件不存在: {}", path),
            ImageError::InvalidFormat(fmt) => write!(f, "无效格式: {}", fmt),
            ImageError::PermissionDenied => write!(f, "权限不足"),
        }
    }
}

impl std::error::Error for ImageError {}

pub type Result<T> = std::result::Result<T, ImageError>;
