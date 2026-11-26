use crate::{PluginError, PluginRequest, PluginResponse, Result};

/// 插件协议处理
pub fn encode_request(method: &str, params: Option<&str>) -> Result<String> {
    if method.is_empty() {
        return Err(PluginError::InvalidProtocol("方法名为空".to_string()));
    }
    
    let params_str = params.map(|s| s.to_string());
    let request = PluginRequest {
        method: method.to_string(),
        params: params_str,
    };
    
    Ok(serde_json::to_string(&request)?)
}

pub fn decode_request(data: &str) -> Result<PluginRequest> {
    if data.is_empty() {
        return Err(PluginError::InvalidProtocol("数据为空".to_string()));
    }
    
    Ok(serde_json::from_str(data)?)
}

pub fn encode_response(success: bool, data: Option<&str>) -> Result<String> {
    let response = PluginResponse {
        success,
        data: data.map(|s| s.to_string()),
        error: None,
    };
    
    Ok(serde_json::to_string(&response)?)
}

pub fn decode_response(data: &str) -> Result<PluginResponse> {
    if data.is_empty() {
        return Err(PluginError::InvalidProtocol("数据为空".to_string()));
    }
    
    Ok(serde_json::from_str(data)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_request() {
        let encoded = encode_request("test.method", Some(r#"{"key":"value"}"#)).unwrap();
        let decoded = decode_request(&encoded).unwrap();
        
        assert_eq!(decoded.method, "test.method");
    }

    #[test]
    fn test_encode_decode_response() {
        let encoded = encode_response(true, None).unwrap();
        let decoded = decode_response(&encoded).unwrap();
        
        assert!(decoded.success);
    }
}
