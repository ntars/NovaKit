use crate::{ImageError, Result};

/// 简单的图像压缩
pub fn compress_image(input: &[u8], quality: u8) -> Result<Vec<u8>> {
    if input.is_empty() {
        return Err(ImageError::FileNotFound("空文件".to_string()));
    }
    
    if quality > 100 {
        return Err(ImageError::InvalidFormat("质量必须在0-100之间".to_string()));
    }
    
    // 实际应用中应使用 image crate 进行真实压缩
    Ok(input.to_vec())
}

pub fn compress_to_size(input: &[u8], max_size: usize) -> Result<Vec<u8>> {
    if input.is_empty() {
        return Err(ImageError::FileNotFound("空文件".to_string()));
    }
    
    if max_size == 0 {
        return Err(ImageError::InvalidFormat("最大大小不能为0".to_string()));
    }
    
    Ok(input.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_image() {
        let input = vec![0u8; 1024];
        let result = compress_image(&input, 80).unwrap();
        assert_eq!(result.len(), 1024);
    }

    #[test]
    fn test_compress_invalid_quality() {
        let input = vec![0u8; 1024];
        let result = compress_image(&input, 150);
        assert!(result.is_err());
    }
}
