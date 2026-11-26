pub mod player;
pub mod transcode;
pub mod component;

use std::io;

#[derive(Debug)]
pub enum MediaError {
    ComponentNotFound(String),
    InvalidFormat(String),
    PermissionDenied,
}

impl std::fmt::Display for MediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaError::ComponentNotFound(comp) => write!(f, "组件未找到: {}", comp),
            MediaError::InvalidFormat(fmt) => write!(f, "无效格式: {}", fmt),
            MediaError::PermissionDenied => write!(f, "权限不足"),
        }
    }
}

impl std::error::Error for MediaError {}

pub type Result<T> = std::result::Result<T, MediaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MediaError::ComponentNotFound("ffmpeg".to_string());
        assert_eq!(format!("{}", err), "组件未找到: ffmpeg");
    }
}
