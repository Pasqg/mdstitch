use std::fs;
use std::path::Path;

pub(crate) fn read_from_file<P: AsRef<Path>>(path: P) -> Option<String> {
    return match fs::read_to_string(path.as_ref()) {
        Err(error) => {
            eprintln!("Could not read '{:?}': {}", path.as_ref(), error);
            return None;
        }
        Ok(content) => Some(content)
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::utils::read_from_file;

    #[test]
    fn returns_none() {
        assert_eq!(read_from_file("non-test-file"), None);
    }

    #[test]
    fn returns_some_content() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/test-file");

        assert_eq!(read_from_file(path.as_path()), Some("A test file.".to_string()));
    }
}