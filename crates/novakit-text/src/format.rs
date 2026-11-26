use crate::{Result, TextError};

pub fn format_json(input: &str) -> Result<String> {
    if input.trim().is_empty() {
        return Err(TextError::EmptyInput);
    }
    
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| TextError::InvalidEncoding(e.to_string()))?;
    
    serde_json::to_string_pretty(&value)
        .map_err(|e| TextError::InvalidEncoding(e.to_string()))
}

pub fn format_xml(input: &str) -> Result<String> {
    if input.trim().is_empty() {
        return Err(TextError::EmptyInput);
    }
    
    let formatted = input.replace("><", ">\n<");
    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json_valid() {
        let input = r#"{"name":"test","value":123}"#;
        let result = format_json(input).unwrap();
        assert!(result.contains("\"name\""));
        assert!(result.contains("\"value\""));
    }

    #[test]
    fn test_format_json_empty() {
        let result = format_json("");
        assert!(result.is_err());
    }

    #[test]
    fn test_format_xml_valid() {
        let input = r#"<root><item>test</item></root>"#;
        let result = format_xml(input).unwrap();
        assert!(result.contains("<root>"));
    }
}
