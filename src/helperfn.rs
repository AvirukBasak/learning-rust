use std::path::Path;

pub fn get_filename(filepath: &str) -> &str {
    Path::new(filepath)
        .file_name()
        .and_then(|s| s.to_str())
        .expect("error getting filename")
}
