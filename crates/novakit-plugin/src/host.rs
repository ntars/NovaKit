use crate::{PluginError, PluginManifest, Result};

/// 插件宿主
pub struct PluginHost {
    plugins_dir: String,
}

impl PluginHost {
    pub fn new(plugins_dir: &str) -> Self {
        Self {
            plugins_dir: plugins_dir.to_string(),
        }
    }

    pub fn load_manifest(&self, name: &str) -> Result<PluginManifest> {
        if name.is_empty() {
            return Err(PluginError::NotFound("插件名为空".to_string()));
        }
        
        // 实际应用中应读取 plugins_dir/name/manifest.json
        Err(PluginError::NotFound(format!("请安装插件: {}", name)))
    }

    pub fn list_plugins(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    pub fn invoke(&self, name: &str, method: &str) -> Result<String> {
        if name.is_empty() || method.is_empty() {
            return Err(PluginError::InvalidProtocol("参数为空".to_string()));
        }
        
        // 实际应用中应调用插件进程
        Ok(format!("插件 {} 未实现方法 {}", name, method))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_new() {
        let host = PluginHost::new("/tmp/plugins");
        assert_eq!(host.plugins_dir, "/tmp/plugins");
    }

    #[test]
    fn test_load_manifest_empty() {
        let host = PluginHost::new("/tmp/plugins");
        let result = host.load_manifest("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invoke_empty() {
        let host = PluginHost::new("/tmp/plugins");
        let result = host.invoke("", "test");
        assert!(result.is_err());
    }
}
