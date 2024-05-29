use crate::utils;

pub(crate) fn stitch(stitch_pattern: &str, index: &str) -> Option<String> {
    let mut merged = String::new();
    for line in index.lines() {
        let trimmed_line = line.trim_start();
        if trimmed_line.len() > stitch_pattern.len() + 3 && trimmed_line.trim_start().starts_with(&stitch_pattern) {
            let (_, path) = trimmed_line.split_at(stitch_pattern.len());
            let path = &path[1..path.len() - 1];

            let content = utils::read_from_file(path);
            if content.is_none() {
                return None;
            }

            let content = stitch(stitch_pattern, content.unwrap().as_str());
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