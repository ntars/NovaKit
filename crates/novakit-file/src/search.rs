use crate::{FileError, Result};

pub fn search_file(path: &str, pattern: &str) -> Result<Vec<String>> {
    let entries = std::fs::read_dir(path)
        .map_err(|_| FileError::NotFound(path.to_string()))?;
    
    let mut matches = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if content.contains(pattern) {
                    matches.push(path.to_string_lossy().into_owned());
                }
            }
        } else if path.is_dir() {
            let sub_matches = search_file(&path.to_string_lossy(), pattern)?;
            matches.extend(sub_matches);
        }
    }
    
    Ok(matches)
}

pub fn search_by_extension(path: &str, ext: &str) -> Result<Vec<String>> {
    let entries = std::fs::read_dir(path)
        .map_err(|_| FileError::NotFound(path.to_string()))?;
    
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext_str) = path.extension() {
                if ext_str == ext {
                    files.push(path.to_string_lossy().into_owned());
                }
            }
        } else if path.is_dir() {
            let sub_files = search_by_extension(&path.to_string_lossy(), ext)?;
            files.extend(sub_files);
        }
    }
    
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_by_extension() {
        let result = search_by_extension("/tmp", "txt");
        assert!(result.is_ok());
    }
}
