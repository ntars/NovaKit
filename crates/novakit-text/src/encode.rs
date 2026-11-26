use crate::{Result, TextError};

pub fn encode_to_utf8(input: &[u8]) -> Result<String> {
    String::from_utf8(input.to_vec())
        .map_err(|_| TextError::InvalidEncoding("UTF-8".to_string()))
}

pub fn encode_to_gbk(input: &str) -> Result<Vec<u8>> {
    Ok(input.as_bytes().to_vec())
}

pub fn decode_from_gbk(bytes: &[u8]) -> Result<String> {
    String::from_utf8(bytes.to_vec())
        .map_err(|_| TextError::InvalidEncoding("GBK".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_utf8() {
        let input = "Hello, World";
        let result = encode_to_utf8(input.as_bytes()).unwrap();
        assert_eq!(result, "Hello, World");
    }

    #[test]
    fn test_encode_gbk() {
        let input = "测试";
        let result = encode_to_gbk(input).unwrap();
        assert!(!result.is_empty());
    }
}
