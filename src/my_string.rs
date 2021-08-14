pub fn cut(string: &String, start_char: char, end_char: char) -> Option<String> {
    let start_index = match string.find(start_char) {
        Some(i) => i,
        None => return None,
    };
    let end_index = match string.find(end_char) {
        Some(i) => i,
        None => return None,
    };
    return Some(string[start_index + 1..end_index].to_string());
}

pub fn get_first_token(string: &String, end_char: char) -> Option<String> {
    return match string.split_once(end_char) {
        Some(s) => Some(s.0.to_string()),
        None => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_string() {
        assert_eq!(
            cut(&String::from("(test)"), '(', ')'),
            Some(String::from("test"))
        );
        assert_eq!(
            get_first_token(&String::from("test,"), ',',),
            Some(String::from("test"))
        );
        assert_eq!(
            get_first_token(&String::from("test("), '(',),
            Some(String::from("test"))
        );
    }
}
