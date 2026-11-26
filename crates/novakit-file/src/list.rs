use crate::{FileError, Result};

pub fn list_directory(path: &str) -> Result<Vec<String>> {
    let entries = std::fs::read_dir(path)
        .map_err(|_| FileError::NotFound(path.to_string()))?;
    
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    
    Ok(files)
}

pub fn list_files_recursive(path: &str, max_depth: usize) -> Result<Vec<String>> {
    if max_depth == 0 {
        return Ok(Vec::new());
    }
    
    let entries = std::fs::read_dir(path)
        .map_err(|_| FileError::NotFound(path.to_string()))?;
    
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            files.push(path.to_string_lossy().into_owned());
        } else if path.is_dir() {
            let sub_files = list_files_recursive(&path.to_string_lossy(), max_depth - 1)?;
            files.extend(sub_files);
        }
    }
    
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_directory() {
        let result = list_directory("/tmp");
        assert!(result.is_ok());
    }
}
