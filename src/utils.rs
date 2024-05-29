use std::fs;

pub(crate) fn read_from_file(path: &str) -> Option<String> {
    return match fs::read_to_string(path) {
        Err(error) => {
            eprintln!("Could not read '{}': {}", path, error);
            return None;
        }
        Ok(content) => Some(content)
    };
}