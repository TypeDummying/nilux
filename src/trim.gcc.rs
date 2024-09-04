
use std::path::Path;

pub fn trim_gcc_headers(path: &Path) -> Option<String> {
    let file_name = path.file_name()?.to_str()?;
    
    if file_name.starts_with("gcc-") {
        let trimmed = file_name.trim_start_matches("gcc-");
        let without_extension = trimmed.trim_end_matches(".h");
        Some(without_extension.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_trim_gcc_headers() {
        let path = PathBuf::from("gcc-stddef.h");
        assert_eq!(trim_gcc_headers(&path), Some("stddef".to_string()));

        let path = PathBuf::from("gcc-limits.h");
        assert_eq!(trim_gcc_headers(&path), Some("limits".to_string()));

        let path = PathBuf::from("regular_file.h");
        assert_eq!(trim_gcc_headers(&path), None);
    }
}
