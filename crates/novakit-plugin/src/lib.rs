pub mod host;
pub mod protocol;

use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginRequest {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug)]
pub enum PluginError {
    NotFound(String),
    InvalidProtocol(String),
    PermissionDenied,
    Serde(SerdeError),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::NotFound(name) => write!(f, "插件未找到: {}", name),
            PluginError::InvalidProtocol(msg) => write!(f, "协议错误: {}", msg),
            PluginError::PermissionDenied => write!(f, "权限不足"),
            PluginError::Serde(e) => write!(f, "序列化错误: {}", e),
        }
    }
}

impl std::error::Error for PluginError {}

impl From<SerdeError> for PluginError {
    fn from(err: SerdeError) -> Self {
        PluginError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, PluginError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_serialization() {
        let manifest = PluginManifest {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: "test plugin".to_string(),
            author: "test".to_string(),
        };
        
        let json = serde_json::to_string(&manifest).unwrap();
        assert!(json.contains("\"name\":\"test\""));
    }
}
