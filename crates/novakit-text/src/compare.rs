use crate::{Result, TextError};
use std::fs;

pub fn compare_files(path1: &str, path2: &str) -> Result<String> {
    let content1 = fs::read_to_string(path1)
        .map_err(|_| TextError::FileNotFound(path1.to_string()))?;
    let content2 = fs::read_to_string(path2)
        .map_err(|_| TextError::FileNotFound(path2.to_string()))?;
    
    let diff = compute_diff(&content1, &content2);
    Ok(diff)
}

fn compute_diff(content1: &str, content2: &str) -> String {
    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();
    
    let mut diff = String::new();
    diff.push_str("=== 差异对比 ===\n\n");
    
    if lines1.len() != lines2.len() {
        diff.push_str(&format!("行数不同: {} vs {}\n", lines1.len(), lines2.len()));
    }
    
    let min_len = std::cmp::min(lines1.len(), lines2.len());
    for i in 0..min_len {
        if lines1[i] != lines2[i] {
            diff.push_str(&format!("第 {} 行不同:\n", i + 1));
            diff.push_str(&format!("- {}\n", lines1[i]));
            diff.push_str(&format!("+ {}\n", lines2[i]));
        }
    }
    
    if diff.contains("不同") == false {
        diff.push_str("无差异");
    }
    
    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_same_files() {
        let content = "line1\nline2\n";
        std::fs::write("/tmp/test1.txt", content).unwrap();
        std::fs::write("/tmp/test2.txt", content).unwrap();
        
        let result = compare_files("/tmp/test1.txt", "/tmp/test2.txt");
        assert!(result.is_ok());
    }
}
