use crate::{ImageError, Result};

/// 简单的图像尺寸调整（实际应使用 image crate）
pub fn resize_image(input: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
    // 简单实现：验证输入大小
    if input.is_empty() {
        return Err(ImageError::FileNotFound("空文件".to_string()));
    }
    
    if width == 0 || height == 0 {
        return Err(ImageError::InvalidFormat("无效尺寸".to_string()));
    }
    
    // 实际应用中应使用 image crate 进行真实缩放
    Ok(input.to_vec())
}

pub fn resize_to_max(input: &[u8], max_size: u32) -> Result<Vec<u8>> {
    if input.is_empty() {
        return Err(ImageError::FileNotFound("空文件".to_string()));
    }
    
    if max_size == 0 {
        return Err(ImageError::InvalidFormat("最大尺寸不能为0".to_string()));
    }
    
    Ok(input.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_image() {
        let input = vec![0u8; 1024];
        let result = resize_image(&input, 100, 100).unwrap();
        assert_eq!(result.len(), 1024);
    }

    #[test]
    fn test_resize_image_empty() {
        let result = resize_image(&[], 100, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_resize_to_max() {
        let input = vec![0u8; 2048];
        let result = resize_to_max(&input, 512).unwrap();
        assert_eq!(result.len(), 2048);
    }
}
