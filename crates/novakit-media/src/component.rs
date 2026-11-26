use crate::{MediaError, Result};

/// 组件检测（ffmpeg/7z等外部工具）
pub fn check_component(name: &str) -> Result<bool> {
    if name.is_empty() {
        return Err(MediaError::InvalidFormat("组件名为空".to_string()));
    }
    
    // 实际应用中应检查系统路径
    Ok(true)
}

pub fn install_component(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(MediaError::InvalidFormat("组件名为空".to_string()));
    }
    
    // 实际应用中应提供安装指引
    Err(MediaError::ComponentNotFound(format!("请手动安装: {}", name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_component() {
        let result = check_component("ffmpeg");
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_component_empty() {
        let result = check_component("");
        assert!(result.is_err());
    }

    #[test]
    fn test_install_component() {
        let result = install_component("ffmpeg");
        assert!(result.is_err());
    }
}
