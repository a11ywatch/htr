/// get the text between two strings
pub fn text_between(search_str: &String, start_str: &String, end_str: &str) -> (String, usize, usize) {
    let start_idx = {
        let start_point = search_str.find(start_str);
        start_point.unwrap() + start_str.len()
    };

    let remaining = &search_str[start_idx..];
    let end_idx = remaining.find(&end_str).unwrap_or(remaining.len());

    (remaining[..end_idx].to_string(), start_idx, end_idx)
}