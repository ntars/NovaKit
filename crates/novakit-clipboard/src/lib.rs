pub mod clipboard;

use std::io;

#[derive(Debug)]
pub enum ClipboardError {
    NotAvailable,
    PermissionDenied,
    InvalidData(String),
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::NotAvailable => write!(f, "剪贴板不可用"),
            ClipboardError::PermissionDenied => write!(f, "权限不足"),
            ClipboardError::InvalidData(data) => write!(f, "无效数据: {}", data),
        }
    }
}

impl std::error::Error for ClipboardError {}

pub type Result<T> = std::result::Result<T, ClipboardError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ClipboardError::NotAvailable;
        assert_eq!(format!("{}", err), "剪贴板不可用");
    }
}
