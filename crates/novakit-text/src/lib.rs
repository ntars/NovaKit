pub mod format;
pub mod encode;
pub mod compare;

#[derive(Debug)]
pub enum TextError {
    EmptyInput,
    InvalidEncoding(String),
    FileNotFound(String),
}

impl std::fmt::Display for TextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextError::EmptyInput => write!(f, "输入为空"),
            TextError::InvalidEncoding(enc) => write!(f, "无效编码: {}", enc),
            TextError::FileNotFound(path) => write!(f, "文件不存在: {}", path),
        }
    }
}

impl std::error::Error for TextError {}

pub type Result<T> = std::result::Result<T, TextError>;
