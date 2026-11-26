use crate::{ClipboardError, Result};

/// 简单的剪贴板操作（实际应使用 tauri-plugin-clipboard）
pub fn get_text() -> Result<String> {
    // 实际应用中应使用 tauri-plugin-clipboard
    Err(ClipboardError::NotAvailable)
}

pub fn set_text(text: &str) -> Result<()> {
    // 实际应用中应使用 tauri-plugin-clipboard
    if text.is_empty() {
        return Err(ClipboardError::InvalidData("文本为空".to_string()));
    }
    
    Ok(())
}

pub fn get_image() -> Result<Vec<u8>> {
    // 实际应用中应使用 tauri-plugin-clipboard
    Err(ClipboardError::NotAvailable)
}

pub fn set_image(data: &[u8]) -> Result<()> {
    if data.is_empty() {
        return Err(ClipboardError::InvalidData("数据为空".to_string()));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_text() {
        let result = set_text("Hello, World!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_text_empty() {
        let result = set_text("");
        assert!(result.is_err());
    }
}
