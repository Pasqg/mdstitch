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

// string.rfind('/') returns the byte position of the character, thus for non-UTF8/ASCII strings
// we cannot just find the latest occurrence with string[string.rfind('/')]
pub(crate) fn last_index_of(string: &str, to_find: char) -> Option<usize> {
    match string.chars().rev().position(|char| char == to_find) {
        Some(last_index) => Some(string.chars().count() - last_index - 1),
        None => None
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::utils::{last_index_of, read_from_file};

    #[test]
    fn returns_none() {
        assert_eq!(read_from_file("non-test-file"), None);
    }

    #[test]
    fn returns_some_content() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/test-file");

        assert_eq!(read_from_file(path.as_path()), Some("@include[dir/other-file]".to_string()));
    }

    #[test]
    fn returns_last_index_of_char() {
        assert_eq!(last_index_of("abcdefg", 'e'), Some(4));
    }

    #[test]
    fn returns_last_index_of_char_with_multiple_occurrences() {
        assert_eq!(last_index_of("eeeeefg", 'e'), Some(4));
    }

    #[test]
    fn returns_none_when_no_char() {
        assert_eq!(last_index_of("eeeeefg", 'a'), None);
    }
}