use crate::utils;

pub(crate) fn stitch(stitch_pattern: &str, index: &str, root_path: &str, verbose: bool) -> Option<String> {
    let mut merged = String::new();
    for line in index.lines() {
        let trimmed_line = line.trim_start();
        if trimmed_line.len() > stitch_pattern.len() + 3 && trimmed_line.trim_start().starts_with(&stitch_pattern) {
            let (_, relative_path_with_file_name) = trimmed_line.split_at(stitch_pattern.len());
            // trims square brackets from start and end
            let relative_path_with_file_name = &relative_path_with_file_name[1..relative_path_with_file_name.len() - 1];
            let (relative_path, _) = match utils::last_index_of(relative_path_with_file_name, '/') {
                Some(index) => relative_path_with_file_name.split_at(index),
                None => ("", relative_path_with_file_name),
            };

            let absolute_path = [root_path, "/", relative_path].concat();
            let absolute_path_with_file_name = [root_path, "/", relative_path_with_file_name].concat();
            let (absolute_path, absolute_path_with_file_name) = match root_path {
                "" => (relative_path, relative_path_with_file_name),
                _ => (absolute_path.as_str(), absolute_path_with_file_name.as_str()),
            };

            if verbose {
                println!("Merging '{}'", absolute_path_with_file_name);
            }

            let content = utils::read_from_file(absolute_path_with_file_name);
            if content.is_none() {
                return None;
            }

            let content = stitch(stitch_pattern, content.unwrap().as_str(), absolute_path, verbose);
            if content.is_none() {
                return None;
            }
            let content = content.unwrap();

            merged.push_str(content.as_str());
        } else {
            merged.push_str(line);
            merged.push('\n');
        }
    }
    Some(merged)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::stitcher::stitch;

    #[test]
    fn should_return_none() {
        let result = stitch("@pattern", "# Index\n\n@pattern[file]", "", false);
        assert_eq!(result, None);
    }

    #[test]
    fn should_merge_following_hierarchy() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/test-file");
        let path = path.as_os_str().to_str().unwrap();

        let result = stitch("@include", format!("# Index\n\n@include[{}]", path).as_str(), "", false);
        assert_eq!(result, Some("# Index\n\nA test file.\n".to_string()));
    }

    #[test]
    fn should_not_merge_if_no_pattern() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/test-file");
        let path = path.as_os_str().to_str().unwrap();

        let result = stitch(".include", format!("# Index\n\n@pattern[{}]", path).as_str(), "", false);
        assert_eq!(result, Some(format!("# Index\n\n@pattern[{}]\n", path)));
    }
}